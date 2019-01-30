createData <- function(n, b1, b2) {
  x <- runif(n, 1, 20)
  y <- rpois(n, lambda = exp(b1 + b2 * x))
  data.frame(x, y)
}

bootstrap <- function(b1) {
  simulatedData <- createData(100, b1, 0)
  fit1 <- glm(formula = y ~ 1, family = poisson, data = simulatedData)
  fit2 <- glm(formula = y ~ x, family = poisson, data = simulatedData)
  fit1$deviance - fit2$deviance
}

test <- function(n, b2) {
  expData <- createData(n, 0.1, b2)
  fit1 <- glm(formula = y ~ 1, family = poisson, data = expData)
  fit2 <- glm(formula = y ~ x, family = poisson, data = expData)
  dev <- fit1$deviance - fit2$deviance
  
  P <- mean(replicate(100, bootstrap(fit1$coefficient[1]) > dev))
  P > 0.05
}

xl <- "Data size"
yl <- "The ratio of failing to reject"
b2 <- c(0.05, 0.07, 0.1, 0.12)
legends <- paste0("b2 = ", b2)
plot(0, 0, type = "n", xlim = c(0, 20), ylim = c(0, 1), xlab = xl, ylab = yl)
n <- 1:20
for(i in 1:4) {
  r <- vector(length = 20)
  for (n in 1:20) {
    r[n] <- mean(replicate(100, test(n, b2[i])))
  }
  lines(1:20, r, type = "b", pch = i) 
}
legend("topright", legend = legends, pch = 1:4)
