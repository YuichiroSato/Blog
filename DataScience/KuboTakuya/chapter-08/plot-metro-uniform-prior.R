dataGenerator <- function(dataSize) {
  rnorm(dataSize, mean = 5, sd = 1)
}

data <- dataGenerator(100)
hist(data, breaks = seq(0, 10, by = 0.1), main = "500 samples", xlab = "data")

likelihood <- function(q) {
  sum(log(dnorm(data, mean = q, sd = 1)))
}

prior <- function(q) {
  dunif(q, 0, 100)
}

posterior <- function(q) {
  likelihood(q) + log(prior(q))
}

develop <- function(q) {
  if (runif(1, 0, 1) < 0.5) {
    q + 0.01
  } else {
    q - 0.01
  }
}

sampling <- function(n, init, evaluator) {
  evaluator <- match.fun(evaluator)
  q <- init
  ql <- evaluator(q)
  samples <- vector(length = n)
  i <- 1
  while(i <= n) {
    p <- develop(q)
    pl <- evaluator(p)
    r <- log(runif(1, 0, 1))
    if (ql < pl || r < pl - ql) {
      q <- p
      ql <- pl
    }
    samples[i] <- p
    i <- i + 1
  }
  samples
}

a <- sampling(10000, 5, likelihood)
hist(a, breaks = seq(0, 10, by = 0.01), main = "MCMC sampling, likelihood", xlab = "q")

b <- sampling(10000, 5, posterior)
hist(b, breaks = seq(0, 10, by = 0.01), main = "MCMC sampling, posterior", xlab = "q")
