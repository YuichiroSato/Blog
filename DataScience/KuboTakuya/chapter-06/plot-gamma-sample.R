dataSize <- 1000
createData <- function(createY) {
  createY <- match.fun(createY)
  x <- runif(dataSize, 0.1, 0.9)
  y <- createY(x)
  data.frame(x, y)
}

dataGenerator <- function()
  createData(function(x) rgamma(dataSize, shape = 4, rate = 4 / exp(0.5 + log(x))))

data <- dataGenerator()
fit <- glm(y ~ log(x), data = data, family = Gamma(link = "log"))
b1 <- fit$coefficients[1]
b2 <- fit$coefficients[2]
phi <- summary(fit)$dispersion

cat("b1", b1, "\n")
cat("b2", b2, "\n")
cat("shape", 1 / phi, "\n")
cat(paste0("rate = ", round(1 / phi, 3), " / exp(", round(b1, 3), " + ", round(b2, 3), " * log(x))\n"))

rate <- function(x) {
  1 / (phi * exp(b1 + b2 * log(x)))
}

xs <- c(0.1, 1.0, 2.0, 3.0, 4.0)
ys <- 1:10
title <- "Gamma regression"
xl <- "y"
yl <- "Probability"
legends <- paste0("x = ", xs)
plot(0, 0, type = "n", xlim = c(1, 10), ylim = c(0.0, 1.0), main = title, xlab = xl, ylab = yl)
for (x in xs) {
  y <- dgamma(ys, shape = 1 / phi, rate = rate(x))
  lines(ys, y, type="l")
  points(ys, y, pch = x)
}
legend("topleft", legend = legends, pch = xs)
