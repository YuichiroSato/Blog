cat("Bayses' theorem: p(m|D) = p(D|m)p(m)/p(D)\n")
cat("                 log p(m|D) = log p(D|m) + log p(m) - log p(D)\n\n")

n <- runif(1, 1.0, 9.0)

rads <- rnorm(10, mean=n[1], sd=2.0)

cat("data D, taken from Normal distribution (mean= ", n[1], ", sd=2.0)\n\n")
rads
summary(rads)
cat("\n")

probD <- function(x) {
  return (dnorm(x, mean=n[1], sd=2.0))
}
logpD <- sum(log(probD(rads)))
cat("p(D), probability of data D taken from Normal distribution (mean= ", n[1], ", sd=2.0) is", exp(logpD), "\n")
cat("log p(D) is", logpD, "\n\n")

cat("m log p(D|m) log p(m) log p(m|D) p(m|D)\n")
res <- 1:10
for (i in 1:10) {
  prob <- function(x) {
    return (dnorm(x, mean=i, sd=2.0))
  }
  pDm <- sum(log(prob(rads)))
  pm <- log(dpois(i, lambda=4.0))
  res[i] <- pDm + pm - logpD
  cat(i, pDm, pm, res[i], exp(res[i]), "\n")  
}
cat("\nmax p(m|D) =", exp(max(res)), ", log p(m|D) =", max(res), "when m =", which.max(res), "\n")
