using Test

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


# <<<TESTS>>>
function main()
  # Number of qubits.
  N = isempty(ARGS) ? 3 : parse(Int, ARGS[1])

  # Simulate random input.
  x = rand(Float32, 2^N)

  # Confusion matrix: [a₀₀ a₀₁ ; a₁₀ a₁₁].
  m::Matrix{Float32} = inv([0.9 0.3; 0.1 0.7])

  print("reshape: ")
  @time res1 = multiply(m, x)

  print("kron: ")
  @time res2::Vector{Float32} = kron(fill(m, N)...) * x
  @test res1 ≈ res2
end

main()

# Example for N=15 on an Apple M1.
# reshape:   0.002849 seconds (157 allocations: 7.754 MiB)
# kron:   2.011607 seconds (37.58 k allocations: 5.335 GiB, 0.43% gc time, 1.92% compilation time) 
