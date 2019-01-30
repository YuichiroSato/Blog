random <- function(x) {
  n <- runif(1, 0.0, 1.0)
  return (n[1])
}

n <- rnorm(1, mean=5.0, sd=2.0)
data <- rnorm(10, mean=n[1], sd=2.0)

cat("data D, taken from Normal distribution (mean= ", n[1], ", sd=2.0)\n\n")
data
summary(data)
cat("\n")

# log p(D|m)
likelihood <- function(m) {
  return (sum(log(dnorm(data, mean=m, sd=2.0))))
}
# log p(m)
probP <- function(m) {
  return (log(dnorm(m, mean=5.0, sd=1.0)))
} 
# log p(D|m)p(m)
eval <- function(m) {
  return (likelihood(m) + probP(m))
}

mean <- 10.0 * random()
prevEval <- eval(mean)
res <- 1:10999
i <- 1

while (i < 11000) {
  newMean <- random() + mean - 0.5 
  newEval <- eval(newMean)
  r <- newEval - prevEval
 
  if (prevEval < newEval || log(random()) < r) {
    prevEval = newEval
    mean = newMean
    res[i] <- newMean
    i <- i + 1
  }
}
# p(m|D)
sampling <- tail(res, n=10000)
summary(sampling)
hist(sampling, breaks=seq(-0.5, 9.5, 0.1))
