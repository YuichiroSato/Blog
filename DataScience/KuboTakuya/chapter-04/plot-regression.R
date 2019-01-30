legends <- c("ideal", "regression")
ltys <- c("dashed", "solid")
for (i in 1:10) {
  y <- rpois(5, lambda = 4)
  data <- data.frame(y)
  
  fit <- glm(formula = y ~ 1, family = poisson, data = data)
  l <- exp(fit$coefficients[1])
  
  title <- paste("Poisson regression, estimated lambda = ", l)
  plot(1:10, dpois(1:10, lambda = 4), ylim = c(0.0, 0.25), type = "l", lty = "dashed", main = title, xlab = "", ylab = "")
  lines(1:10, dpois(1:10, lambda = l), type = "l")
  legend("topright", legend = legends, lty = ltys)
}
