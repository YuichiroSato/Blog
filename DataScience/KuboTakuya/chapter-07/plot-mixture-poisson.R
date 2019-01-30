linkFunction <- function(b1, b2, x) {
  function(r) {
    exp(b1 + b2 * x + r)
  }
}

distribution <- function(s, linkFunction) {
  l <- match.fun(linkFunction)
  function(y) {
    sum <- 0
    for (r in seq(-100, 100, by = 0.1)) {
      sum <- sum + dpois(y, l(r)) * dnorm(r, mean = 0, sd = s) * 0.1
    }
    sum
  }
}

xl <- "y"
yl <- "Probability"
xs <- 1:5
ys <- 0:10
b1 <- 0.1
legends = paste0("x = ", xs)
for (b2 in seq(0.1, 0.5, by = 0.1)) {
  for (s in c(0.5, 1.0, 1.5, 2.0)) {
    title <- paste0("lambda = exp(", b1, " + ", b2, " * x + r, r ~ N(0, ", s, ")")
    plot(0, 0, type = "n", xlim = c(0, 10), ylim = c(0.0, 0.5), main = title, xlab = xl, ylab = yl)
    for (i in 1:5) {
      l <- linkFunction(b1, b2, xs[i])
      d <- distribution(s, l)
      lines(ys, d(ys), type = "l")
      points(ys, d(ys), pch = i)
    }
    legend("topright", legend = legends, pch = 1:5)
  }
}
