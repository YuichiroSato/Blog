N <- 10
dataSize <- 100
createData <- function(createY) {
  createY <- match.fun(createY)
  x <- runif(dataSize, 0, 20)
  c <- runif(dataSize, 0, 20)
  y <- createY(x)
  data.frame(x, c, y)
}

dataGenerator <- function()
  createData(function(x) rbinom(dataSize, N, 1 / (1 + exp(5.0 - 0.5 * x))))

data <- dataGenerator()
calcAIC <- function(formula) {
  fit <- glm(formula, data = data, family = binomial)
  fit$aic
}

cat("model\t\taic\n")
cat("null\t\t", calcAIC(cbind(y, N - y) ~ 1), "\n")
cat("x\t\t", calcAIC(cbind(y, N - y) ~ x), "\n")
cat("c\t\t", calcAIC(cbind(y, N - y) ~ c), "\n")
cat("x + c\t\t", calcAIC(cbind(y, N - y) ~ x + c), "\n")
cat("xc\t\t", calcAIC(cbind(y, N - y) ~ x : c), "\n")
cat("x + xc\t\t", calcAIC(cbind(y, N - y) ~ x + x : c), "\n")
cat("c + xc\t\t", calcAIC(cbind(y, N - y) ~ c + x : c), "\n")
cat("x + c + xc\t", calcAIC(cbind(y, N - y) ~ x * c), "\n")
