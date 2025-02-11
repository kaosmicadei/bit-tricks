module NRankMul

export bitswap0, multiply

"""Swaps the *i*-th bit from a number `n` with its least significant bit. 
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
    multiply(matrix2x2, vector2N)

Memory and time efficent multiplication of a 2×2 matix `M` by a 2^N vector `v`
similar to

    kron(fill(M, N)...) * v

# Examples
```julia-repl
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
  vector_length = length(vector)
  rank = log2(vector_length) |> Int64

  vector_indices = 0:(vector_length-1)
  tensor_indices = 0:(rank-1)

  f = (acc, i) -> matrix2x2 * reshape(acc[bitswap0.(i, vector_indices) .+ 1], 2,:)
  foldl(f, tensor_indices; init=copy(vector2N)) |> transpose |> vec
end

end # module NRankMul
