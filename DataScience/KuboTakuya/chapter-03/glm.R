d <- read.csv("data3a.csv")
cat("summary of input data\n")
summary(d)

fit <- glm(y ~ x, data = d, family = poisson)

b1 <- fit$coefficients[1]
b2 <- fit$coefficients[2]

cat("\nlink function\n")
cat("exp(", b1, " + ", b2, "x)\n")

linkFunction <- function(x) {
  exp(b1 + b2 * x)
}

ps <- 0:20
xl <- "y"
yl <- "probability"
legends <- c("x = 0","x = 5","x = 10", "x = 15", "x = 20")
pchs <- c(0, 5, 10, 15, 20)
title <- paste("lambda = exp(", b1, "+", b2, "x)")
plot(0, 0, type = "n", xlim = c(0, 20), ylim = c(0.0, 0.25), main = title, xlab = xl, ylab = yl)
for (x in seq(0, 20, by = 5)) {
  d <- function(y) {
    dpois(y, lambda = linkFunction(x))
  }
  lines(ps, d(ps), type = "l")
  points(ps, d(ps), pch = x)
}
legend("topright", legend = legends, pch = pchs)
