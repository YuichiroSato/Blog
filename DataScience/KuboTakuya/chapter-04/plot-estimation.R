createHistogram <- function(n) {
  lambdas <- vector(length = 1000)
  for (i in 1:1000) {
    y <- rpois(n, lambda = 4)
    data <- data.frame(y)
    
    fit <- glm(formula = y ~ 1, family = poisson, data = data)
    lambdas[i] <- exp(fit$coefficients[1])
  }
  title <- paste("Number of data =", n)
  hist(lambdas[lambdas <= 10], breaks = seq(0, 10, by = 0.01), main = title, xlab = "Estimeted lambda")
}

for (i in c(1, 5, 10, 100, 200)) {
  createHistogram(i)
}
