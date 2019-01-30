linkFunction <- function(b1, b2, x) {
  function(r) {
    1 / (1 + exp(-b1 - b2 * x - r))
  }
}

l <- linkFunction(-0.5, 0.3, 2)

cat("r\ty=0\t\ty=1\t\ty=2\t\ty=3\n")
sum <- vector(length = 4)
for (r in seq(-5, 5, by = 0.5)) {
  q <- l(r)
  cat(r, "\t")
  for (y in 0:3) {
    b <- dbinom(y, 3, q)
    n <- dnorm(r, mean = 0, sd = 2)
    mixture <- b * n
    sum[y + 1] <- sum[y + 1] + mixture * 0.5
    cat(mixture, "\t")
  }
  cat("\n")
}
cat("sum\t", sum[1], "\t", sum[2], "\t", sum[3], "\t", sum[4], "\n")
