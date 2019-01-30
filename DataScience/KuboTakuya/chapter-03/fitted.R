d <- read.csv("data3a.csv")

fit <- glm(y ~ x, data = d, family = poisson)

b1 <- fit$coefficients[1]
b2 <- fit$coefficients[2]

linkFunction <- function(x) {
  exp(b1 + b2 * x)
}

fitted <- function(x, y) {
  dpois(y, lambda=linkFunction(x))
}

cat("p(y | exp(a + bx)) where p is Poisson distribution\n")
cat("a =", b1, "b =", b2, "\n\n")

y <- 1:20
max <- numeric(length=20)
for (x in 1:20) {
  at <- fitted(x, y)
  cat("when x = ", x, ", y = ", which.max(at), " is most probable, probability is ", max(at), "\n")
  max[x] <- which.max(at)
}
title <- paste0("p(y|exp(", b1, " + ", b2, "x))")
xl <- "x"
yl <- "the most probable y"
plot(1:20, max, type = "b", main = title, xlab = xl, ylab = yl)
