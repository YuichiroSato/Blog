dataGenerator <- function(dataSize) {
  gen <- function() {
    m <- 3
    if (runif(1, 0, 1) < 0.5) {
      m <- 7
    }
    q <- rnorm(1, mean = m, sd = 0.1)
    rnorm(1, mean = q[1], sd = 1)
  }
  data <- vector(length = dataSize)
  for (i in seq(1, dataSize + 1)) {
    data[i] <- gen()
  }
  data
}

data <- dataGenerator(100)
hist(data, breaks = seq(0, 10, by = 0.1), main = "500 samples", xlab = "data")

likelihood <- function(q) {
  sum(log(dnorm(data, mean = q, sd = 1)))
}

prior <- function(q) {
  (dnorm(q, mean = 3, sd = 0.1) + dnorm(q, mean = 7, sd = 0.1)) / 2
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

a1 <- sampling(25000, 3, likelihood)
a2 <- sampling(25000, 7, likelihood)
hist(c(a1, a2), breaks = seq(0, 10, by = 0.01), main = "MCMC sampling, lilelihood", xlab = "q")

b1 <- sampling(25000, 3, posterior)
b2 <- sampling(25000, 7, posterior)
hist(c(b1, b2), breaks = seq(0, 10, by = 0.01), main = "MCMC sampling, posterior", xlab = "q")
