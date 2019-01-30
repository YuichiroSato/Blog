linkFunction <- function(b1, b2) {
  function(x) {
    exp(b1 + b2 * x)
  }
}

createData <- function(n, b1, b2) {
  x <- runif(n, 1, 20)
  y <- rpois(n, lambda = linkFunction(b1, b2)(x))
  data.frame(x, y)
}

plotGraph <- function(data, fit1, fit2) {
  plot(data$x, data$y, type = "p", main = "Bootstrap", xlab = "x", ylab = "y")
  lines(0:20, linkFunction(fit1$coefficient[1], 0)(0:20), lty = "solid")
  lines(0:20, linkFunction(fit2$coefficient[1], fit2$coefficient[2])(0:20), lty = "dashed")
  nl <- paste0("y = exp(", round(fit1$coefficient[1], 3), ")")
  xl <- paste0("y = exp(", round(fit2$coefficient[1], 3), " + ", round(fit2$coefficient[2], 3), " * x)")
  legend("topleft", legend = c(nl, xl), lty = c("solid", "dashed"))
}

bootstrap <- function(b1) {
  simulatedData <- createData(100, b1, 0)
  fit1 <- glm(formula = y ~ 1, family = poisson, data = simulatedData)
  fit2 <- glm(formula = y ~ x, family = poisson, data = simulatedData)
  fit1$deviance - fit2$deviance
}

showBootstrap <- function(dataSize, b1, b2) {
  d <- createData(dataSize, b1, b2)
  fit1 <- glm(formula = y ~ 1, family = poisson, data = d)
  fit2 <- glm(formula = y ~ x, family = poisson, data = d)
  dev <- fit1$deviance - fit2$deviance
  
  devs <- replicate(1000, bootstrap(fit1$coefficients[1]))
  cat("data size", dataSize, paste0(", lambda = exp(", b1, " + ", b2, "x)"), "\n")
  summary(d$y)
  P <- sum(devs >= dev) / 1000
  cat("P =", P, "\n")
  if (P < 0.05) {
    cat("The null hypothesis is rejected.\n\n")
  } else {
    cat("Failling to reject the null hypothesis.\n\n")
  }
  plotGraph(d, fit1, fit2)
}

showBootstrap(5, 0.01, 0.03)
showBootstrap(5, 0.01, 0.1)
showBootstrap(20, 0.01, 0.03)
showBootstrap(20, 0.01, 0.1)
