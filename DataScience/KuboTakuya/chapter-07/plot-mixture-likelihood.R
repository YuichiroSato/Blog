dataSize <- 50
N <- 20

linkFunction <- function(b1, b2, x) {
  function(r) {
    1 / (1 + exp(-b1 - b2 * x - r))
  }
}

dmixture <- function(b1, b2, s) {
  function(y, x) {
    l <- linkFunction(b1, b2, x)
    sum <- 0
    for (r in seq(-100, 100, by = 0.1)) {
      sum <- sum + dbinom(y, N, l(r)) * dnorm(r, mean = 0, sd = s) * 0.1
    }
    sum
  }
}

likelihood <- function(b1, b2, s) {
  d <- dmixture(b1, b2, s)
  function (y, x) {
    d(y, x)
  }
}

logLikelihood <- function(data, b1, b2) {
  function (s) {
    sl <- 0
    ll <- likelihood(b1, b2, s)
    for (i in 1:dataSize) {
      sl <- sl + log(ll(data$y[i], data$x[i]))
    }
    sl
  }
}

createData <- function() {
  x <- runif(dataSize, 0, 5)
  y <- vector(length = dataSize)
  for (i in 1:dataSize) {
    r <- rnorm(1, mean = 0, sd = 1)
    l <- linkFunction(-2, 1, x[i])
    y[i] <- rbinom(1, N, l(r))
  }
  data.frame(x, y)
}

data <- createData()
xl <- "s"
yl <- "Lilelihood"
ss <- seq(0.2, 2, by = 0.2) 
b2s <- seq(-2, 2, by = 1)
legends <- paste0("b2 = ", b2s)
for (b1 in seq(-2, 2, by = 1)) {
  title <- paste0("logit(q) = ", b1, " + b2 * x + r, r ~ N(0, s)")
  plot(0, 0, type = "n", xlim = c(0, 2), ylim = c(-1000, 0), main = title, xlab = xl, ylab = yl)
  for (i in 1:5) {
    ls <- logLikelihood(data, b1, b2s[i])(ss)
    lines(ss, ls, type = "l")
    points(ss, ls, pch = i)
  }
  legend("topleft", legend = legends, pch = 1:5)
}
