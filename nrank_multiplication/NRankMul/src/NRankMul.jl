module NRankMul

export bit_swap, multiply

"""Swaps the *i*-th bit from a number `n` with its least significant bit. 
Adapted from:
    http://graphics.stanford.edu/~seander/bithacks.html#SwappingBitsXOR
"""
function bit_swap(bit_index::Int64, number::Int64)::Int64
  # Checks if the target bits are the different.
  x = (number ⊻ (number >> bit_index)) & 1

  # Flip the target and the first bit.
  number ⊻ ((x << bit_index) | x)
end

function multiply(matrix::Matrix{Float32}, vector::Vector{Float32})::Vector{Float32}
  vector_length = length(vector)
  num_qubits = log2(vector_length) |> Int64
  acc = vector |> copy |> v -> reshape(v, 2, :)
  for i in 0:(num_qubits-1)
    acc .= matrix * reshape(acc[bit_swap.(i, 0:(vector_length-1)).+1], 2, :)
  end
  transpose(acc) |> vec
end

end # module NRankMul
