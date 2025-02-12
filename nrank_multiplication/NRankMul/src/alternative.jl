#=
  Not the best solution but worthy to keep for the algorithm. It's memory
  efficent but too slow.
=#
function multiply_alt(matrix2x2::Matrix{Float32}, vector2N::Vector{Float32})::Vector{Float32}
  @assert size(matrix2x2) == (2,2) "Matrix must be 2x2."
  len = length(vector2N)
  @assert len & (len - 1) == 0 "Vector must be a power of 2."

  log_m = log.(matrix2x2)

  indices = 0:(len - 1)
  rank = log2(len) |> UInt32

  f(lin, col) = begin
    a = rank-count_ones(lin | col)
    b = count_ones((lin ⊻ col) & col)
    c = count_ones((lin ⊻ col) & lin)
    d = count_ones(lin & col)
    a * log_m[1,1] + b * log_m[1,2] + c * log_m[2,1] + d * log_m[2,2] |> sum |> exp
  end

  g(acc, (col, val)) = acc .+ val .* map(lin -> f(lin, col-1), indices)

  foldl(g, enumerate(vector2N), init=zeros(Float32,len))
end
