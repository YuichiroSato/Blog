N <- 10
dataSize <- 100
createData <- function(createY) {
  createY <- match.fun(createY)
  x <- runif(dataSize, 0, 20)
  y <- createY(x)
  data.frame(x, y)
}

dataGenerator <- function()
  createData(function(x) rbinom(dataSize, N, 1 / (1 + exp(5.0 - 0.5 * x))))

data <- dataGenerator()
fit <- glm(cbind(y, N - y) ~ x, data = data, family = binomial)
b1 <- fit$coefficients[1]
b2 <- fit$coefficients[2]

linkFunction <- function(x) 1 / (1 + exp(-b1 - b2 * x))

xs <- seq(5, 20, by = 3)
ys <- 0:N
title <- "Logistic regression"
xl <- "y"
yl <- "Probability"
legends <- paste0("x = ", xs)
plot(0, 0, type = "n", xlim = c(0, N), ylim = c(0.0, 1.0), main = title, xlab = xl, ylab = yl)
for (x in xs) {
  q <- linkFunction(x)
  y <- dbinom(ys, N, q)
  lines(ys, y, type="l")
  points(ys, y, pch = x)
}
legend("topleft", legend = legends, pch = xs)
