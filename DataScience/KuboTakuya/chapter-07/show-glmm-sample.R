library('glmmML')

dataSize <- 100
N <- 20

linkFunction <- function(b1, b2, x) {
  function(r) {
    1 / (1 + exp(-b1 - b2 * x - r))
  }
}

createData <- function() {
  x <- runif(dataSize, 0, 5)
  y <- vector(length = dataSize)
  for (i in 1:dataSize) {
    r <- rnorm(1, mean = 0, sd = 2)
    l <- linkFunction(-2, 1, x[i])
    y[i] <- rbinom(1, N, l(r))
  }
  id <- 1:dataSize
  data.frame(x, y, id)
}

data <- createData()
glmmML(cbind(y, N - y) ~ x, data = data, family = binomial, cluster = id)
