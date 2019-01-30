linkFunction <- function(b1, b2, x) {
  function(r) {
    exp(b1 + b2 * x + r)
  }
}

distribution <- function(y, s, linkFunction) {
  l <- match.fun(linkFunction)
  function(r) {
    dpois(y, l(r)) * dnorm(r, mean = 0, sd = s)
  }
}

xl <- "r"
yl <- "Probability"
ys <- seq(0, 8, by = 2)
rs <- seq(-5, 5, by = 0.5)
b1 <- 0.1
legends = paste0("y = ", ys)
for (b2 in seq(0.1, 0.5, by = 0.1)) {
  for (s in c(0.5, 1.0, 1.5, 2.0)) {
    l <- linkFunction(b1, b2, 2)
    title <- paste0("lambda  = exp(", b1, " + ", b2, " * 2 +  r), r ~ N(0, ", s, ")")
    plot(0, 0, type = "n", xlim = c(-5, 5), ylim = c(0.0, 0.2), main = title, xlab = xl, ylab = yl)
    for (i in 1:5) {
      d <- distribution(ys[i], s, l)
      lines(rs, d(rs), type = "l")
      points(rs, d(rs), pch = i)
    }
    legend("topright", legend = legends, pch = 1:5)
  }
}
