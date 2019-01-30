dataSize <- 20
N <- 8

data <- rbinom(dataSize, N, 0.45)
summary(data)
cat("Analytically, q =", sum(data) / (dataSize * N), "\n\n")

likelihood <- function (q) {
  sum(log(dbinom(data, N, q)))
}

develop <- function(q) {
  if (runif(1, 0, 1) < 0.5) {
    q + 0.01
  } else {
    q - 0.01
  }
}
  
q <- round(0.3 + runif(1, 0, 0.3), 2)
ql <- likelihood(q)
i <- 0
cat("start q =", q, "\n")
while(i < 30) {
  p <- develop(q)   
  pl <- likelihood(p)
  r <- log(runif(1, 0, 1))
  if (ql < pl || r < pl - ql) {
    q <- p
    ql <- pl
  }
  cat("q =", q, "likelihood", ql, "\n")
  i <- i + 1
}
cat("last q =", q, "\n")
