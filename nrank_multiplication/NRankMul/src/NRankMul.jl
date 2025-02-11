module NRankMul

export bitswap0, multiply, multiply_alt

"""
    bitswap0(bit_index::Int64, number::Int64)::Int64

Swaps the *i*-th bit from a number `n` with its least significant bit. 
Adapted from:
    http://graphics.stanford.edu/~seander/bithacks.html#SwappingBitsXOR
"""
function bitswap0(bit_index::Int64, number::Int64)::Int64
  # Checks if the target bits are the different.
  x = (number ⊻ (number >> bit_index)) & 1

  # Flip the target and the first bit.
  number ⊻ ((x << bit_index) | x)
end

"""
    multiply(matrix2x2::Matrix{Float32}, vector2N::Vector{Float32})::Vector{Float32}

Memory and time efficent multiplication of a 2×2 matix `M` by a 2^N vector `v`
similar to

    kron(fill(M, N)...) * v

# Examples
```jldoctest
julia> multiply([1 2; 3 4], collect(1:8))
8-element Vector{Int64}:
  153
  351
  345
  791
  333
  763
  749
 1715
```
"""
function multiply(matrix2x2::Matrix{Float32}, vector2N::Vector{Float32})::Vector{Float32}
  # Get the number of indices.
  vector_length = length(vector2N)
  rank = log2(vector_length) |> Int64

  vector_indices = 0:(vector_length-1)
  tensor_indices = 0:(rank-1)

  f = (acc, i) -> matrix2x2 * reshape(acc[bitswap0.(i, vector_indices) .+ 1], 2,:)
  foldl(f, tensor_indices; init=copy(vector2N)) |> transpose |> vec
end

function multiply_alt(matrix2x2::Matrix{Float32}, vector2N::Vector{Float32})::Vector{Float32}
  @assert size(matrix2x2) == (2,2) "Matrix must be 2x2."
  len = length(vector2N)
  @assert len & (len - 1) == 0 "Vector must be a power of 2."

  rank = log2(len) |> UInt32
  indices = 0:(len - 1)

  # Assuming matrix like [m₀₀ m₀₁; m₁₀ m₁₁]
  log_m00 = log(matrix2x2[1,1])
  log_m01 = log(matrix2x2[1,2])
  log_m10 = log(matrix2x2[2,1])
  log_m11 = log(matrix2x2[2,2])

  acc = zeros(Float32,len)
  for out_idx in indices
    for in_idx in indices
      cm00 = rank - count_ones(in_idx | out_idx)
      cm01 = count_ones((in_idx ⊻ out_idx) & in_idx)
      cm10 = count_ones((in_idx ⊻ out_idx) & out_idx)
      cm11 = count_ones(in_idx & out_idx)
      
      s = cm00 * log_m00 + cm01 * log_m01 + cm10 * log_m10 + cm11 * log_m11
      acc[out_idx + 1] += exp(s) * vector2N[in_idx + 1]
    end
  end

  acc
end

end # module NRankMul
