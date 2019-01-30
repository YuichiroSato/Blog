n <- runif(1, 1, 10)

rads <- rpois(20, lambda=n[1])

cat("input data taken from Poisson distribution (lambda = ", n[1], ")\n\n")
rads
summary(rads)
cat("\n")

cat("likelihood of given data at\n")
res <- vector(length = 10)
for (i in 1:10) {
  prob <- function(x) {
    dpois(x, lambda=i)
  }
  res[i] <- sum(log(prob(rads)))
  cat("lambda = ", i, " ", res[i], "\n")
}

cat("\nmaximum likelihood = ", max(res), "when lambda = ", which.max(res), "\n")
