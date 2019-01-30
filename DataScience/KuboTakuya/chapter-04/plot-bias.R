dataSize <- 100
expSize <- 1000

createData <- function(createY) {
  createY <- match.fun(createY)
  x <- runif(dataSize, 0, 10)
  c <- runif(dataSize, 0, 10)
  y <- createY(x, c)
  data.frame(x, c, y)
}

createIndipendentData <- function() createData(function(x, c) rpois(dataSize, lambda = 4))
createCorrelatedXData <- function() createData(function(x, c) rpois(dataSize, lambda = exp(0.1 + 0.2 * x)))
createCorrelatedCData <- function() createData(function(x, c) rpois(dataSize, lambda = exp(0.1 + 0.2 * c)))
createCorrelatedXCData <- function() createData(function(x, c) rpois(dataSize, lambda = exp(0.1 + 0.2 * x + 0.2 * c)))

logLikelihood <- function(genData, b1, b2, b3) {
  genData <- match.fun(genData)
  data <- genData()
  prob <- vector(length = dataSize)
  for (j in 1:dataSize) {
    prob[j] <- log(dpois(data$y[j], lambda = exp(b1 + b2 * data$x[j] + b3 * data$c[j])))
  }
  sum(prob)
}

meanLogLikelihood <- function(genData, b1, b2, b3) {
  mean(replicate(dataSize, logLikelihood(genData, b1, b2, b3)))
}

calculateAic <- function(d, genData, f, n) {
  fit <- glm(formula = f, family = poisson, data = d)
  co <- append(fit$coefficients, replicate(2, 0))
  names(co) <- n
  
  meanLogLik <- meanLogLikelihood(genData, co["b1"], co["b2"], co["b3"])
  bias <- logLik(fit) - meanLogLik
  aic <- -2 * (logLik(fit) - length(fit$coefficients))
  result <- c(logLik(fit), meanLogLik, bias, aic)
  names(result) <- c("logLik", "meanLogLik", "bias", "aic")
  result
}

calculateMeanAic <- function(genData, f, n) {
  genData <- match.fun(genData)
  m <- replicate(expSize, calculateAic(genData(), genData, f, n))
  apply(m, 1, mean)
}

showAic <- function(genData) {
  r1 <- calculateMeanAic(genData, y ~ 1, c("b1","b2","b3"))
  r2 <- calculateMeanAic(genData, y ~ x, c("b1","b2","b3"))
  r3 <- calculateMeanAic(genData, y ~ c, c("b1","b3","b2"))
  r4 <- calculateMeanAic(genData, y ~ x + c, c("b1","b2","b3"))
  
  cat("\t\t\tnull\t\tx\t\tc\t\txc\n")
  cat("log likelihood\t\t", r1["logLik"], "\t", r2["logLik"], "\t", r3["logLik"], "\t", r4["logLik"], "\n")
  cat("mean log likelihood\t", r1["meanLogLik"], "\t", r2["meanLogLik"], "\t", r3["meanLogLik"], "\t", r4["meanLogLik"], "\n")
  cat("bias\t\t\t", r1["bias"], "\t", r2["bias"], "\t", r3["bias"], "\t", r4["bias"], "\n")
  cat("aic\t\t\t", r1["aic"], "\t", r2["aic"], "\t", r3["aic"], "\t", r4["aic"], "\n\n")
}

cat("indipendent data\n")
showAic(createIndipendentData)

cat("correlated X data\n")
showAic(createCorrelatedXData)

cat("correlated C data\n")
showAic(createCorrelatedCData)

cat("correlated XC data\n")
showAic(createCorrelatedXCData)
