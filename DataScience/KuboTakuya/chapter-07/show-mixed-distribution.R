linkFunction <- function(b1, b2) {
  function(x, r) {
    1 / (1 + exp(-b1 - b2 * x - r))
  }
}

l <- linkFunction(-0.5, 0.3)

cat("r\tq\t\tdbinom\t\tdnorm\t\tmixed\n")
for (r in seq(-1, 1, by = 0.1)) {
  q <- l(0.8, r)
  b <- dbinom(7, 20, q)
  n <- dnorm(r, mean = 0, sd = 1)
  mixed <- b * n
  cat(r, "\t", q, "\t", b, "\t", n, "\t", mixed, "\n")
}
