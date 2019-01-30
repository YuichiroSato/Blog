N <- 20

linkFunction <- function(b1, b2, x) {
  function(r) {
    1 / (1 + exp(-b1 - b2 * x - r))
  }
}

distribution <- function(y, linkFunction) {
  l <- match.fun(linkFunction)
  function(r) {
    dbinom(y, N, l(r)) * dnorm(r, mean = 0, sd = 2)
  }
}


xl <- "r"
yl <- "Probability"
ys <- seq(0, 20, by = 5)
rs <- seq(-5, 5, by = 0.5)
legends = paste0("y = ", ys)
for (b1 in seq(-0.5, 0.5, by = 0.2)) {
  for (b2 in seq(-0.5, 0.5, by = 0.2)) {
    for (x in 0:3) {
      l <- linkFunction(b1, b2, x)
      title <- paste0("logit(q) = ", b1, " + ", b2, " * ", x, " + r, r ~ N(0, 2)")
      plot(0, 0, type = "n", xlim = c(-5, 5), ylim = c(0.0, 0.1), main = title, xlab = xl, ylab = yl)
      for (i in 1:5) {
        d <- distribution(ys[i], l)
        lines(rs, d(rs), type = "l")
        points(rs, d(rs), pch = ys[i])
      }
      legend("topright", legend = legends, pch = ys)
    }
  }
}
