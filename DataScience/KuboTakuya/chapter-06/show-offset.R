dataSize <- 100
createData <- function(n, createY) {
  createY <- match.fun(createY)
  x <- runif(dataSize, 0, 5)
  A <- runif(dataSize, n, 10 * n)
  y <- createY(x, A)
  data.frame(x, A, y)
}

linkFunction <- function(x, A) rpois(dataSize, exp(0.1 + 0.3 * x + log(A)))
dataGenerator1 <- function() createData(1, linkFunction)
dataGenerator2 <- function() createData(100, linkFunction)

doGlm <- function(dataGen) {
  dataGen <- match.fun(dataGen)
  data <- dataGen()
  fit <- glm(y ~ x, offset = log(A), data = data, family = poisson)
  c(fit$coefficients[1], fit$coefficients[2])
}

showResult <- function(dataGen) {
  dataGen <- match.fun(dataGen)
  m <- replicate(100, doGlm(dataGen))
  meanm <- apply(m, 1, mean)
  sdm <- apply(m, 1, sd)
  cat("mean b1", meanm[1], "\n")
  cat("mean b2", meanm[2], "\n")
  cat("sd b1", sdm[1], "\n")
  cat("sd b2", sdm[2], "\n")
}
  
cat("case1\n")
showResult(dataGenerator1)
cat("\ncase2\n")
showResult(dataGenerator2)
