N <- 20

linkFunction <- function(b1, b2, x) {
  function(r) {
    1 / (1 + exp(-b1 - b2 * x - r))
  }
}

distribution <- function(s, linkFunction) {
  l <- match.fun(linkFunction)
  function(y) {
    sum <- 0
    for (r in seq(-100, 100, by = 0.1)) {
      sum <- sum + dbinom(y, N, l(r)) * dnorm(r, mean = 0, sd = s) * 0.1
    }
    sum
  }
}


xl <- "y"
yl <- "Probability"
xs <- c(-5,-2,0,2,5) 
ys <- 0:20
legends = paste0("x = ", xs)
for (b1 in seq(-0.5, 0.5, by = 0.2)) {
  for (b2 in seq(-0.5, 0.5, by = 0.2)) {
    for (s in c(0.1, 0.5, 1, 1.5, 2)) {
      title <- paste0("logit(q) = ", b1, " + ", b2, " * x + r, r ~ N(0, ", s, ")")
      plot(0, 0, type = "n", xlim = c(0, 20), ylim = c(0.0, 0.5), main = title, xlab = xl, ylab = yl)
      for (i in 1:5) {
        l <- linkFunction(b1, b2, xs[i])
        d <- distribution(s, l)
        lines(ys, d(ys), type = "l")
        points(ys, d(ys), pch = i)
      }
      legend("topright", legend = legends, pch = 1:5)
    }
  }
}
