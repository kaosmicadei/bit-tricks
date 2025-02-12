module NRankMul

export bitswap0, multiply

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
    multiply(m::Matrix{Float32}, v::Vector{Float32})::Vector{Float32}

Memory and time efficent multiplication of a 2×2 matix `M` by a 2^N vector `v`
similar to

    kron(fill(M, N)...) * v

This function was originally designed for error correction of quantum
measurementsto applying a confusion matrix to a vector of measurements.

# Examples
```jldoctest
julia> multiply([Float32(1) 2; 3 4], collect(Float32(1):8))
8-element Vector{Int64}:
  153.0
  351.0
  345.0
  791.0
  333.0
  763.0
  749.0
 1715.0
```
"""
function multiply(m::Matrix{Float32}, v::Vector{Float32})::Vector{Float32}
  @assert size(m) == (2,2) "Matrix must be 2x2."

  vector_length = length(v)
  @assert vector_length & (vector_length - 1) == 0 "Vector length must be a power of 2."

  # Get the number of indices.
  rank = log2(vector_length) |> Int64

  vector_indices = 0:(vector_length-1)
  tensor_indices = 0:(rank-1)

  f = (acc, i) -> m * reshape(acc[bitswap0.(i, vector_indices) .+ 1], 2,:)
  foldl(f, tensor_indices; init=copy(v)) |> transpose |> vec
end

end # module NRankMul
