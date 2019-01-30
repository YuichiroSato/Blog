N <- 100
dataSize <- 100
createData <- function(createY) {
  createY <- match.fun(createY)
  x <- runif(dataSize, 0, 200)
  y <- createY(x)
  data.frame(x, y)
}

dataGenerator <- function()
  createData(function(x) rbinom(dataSize, N, 1 / (1 + exp(5.0 - 0.05 * x))))

data <- dataGenerator()
fit <- glm(cbind(y, N - y) ~ x, data = data, family = binomial)
b1 <- fit$coefficients[1]
b2 <- fit$coefficients[2]

linkFunction <- function(x) 1 / (1 + exp(-b1 - b2 * x))

xs <- 1:200
ys <- 0:N
title <- "Logistic regression"
xl <- "x"
yl <- "The most probable y"
maxY <- numeric(length = 200)
for (x in xs) {
  q <- linkFunction(x)
  maxY[x] <- which.max(dbinom(ys, N, q)) - 1
}
plot(0, 0, type = "n", xlim = c(0, 200), ylim = c(0, N), main = title, xlab = xl, ylab = yl)
lines(xs, maxY, type="l")
