N <- 20

linkFunction <- function(b1, b2) {
  function(x, r) {
    1 / (1 + exp(-b1 - b2 * x - r))
  }
}

distribution <- function(x, r, linkFunction) {
  l <- match.fun(linkFunction)
  function(y) {
    dbinom(y, N, l(x, r))
  }
}

rs <- -2:2
ys <- 0:N
pchs <- 1:5
xl <- "y"
yl <- "Probability"
legends <- paste0("r = ", rs)
for (b1 in c(-0.5, 0.5, by = 0.2)) {
  for (b2 in c(-0.5, 0.5, by = 0.2)) {
    l <- linkFunction(b1, b2)
    title <- paste0("logit(q) = ", b1, " + ",b2," * 2"," + r")
    plot(0, 0, type = "n", xlim = c(0, N), ylim = c(0.0, 1.0), main = title, xlab = xl, ylab = yl)
    for (i in 1:5) {
      d <- distribution(2, rs[i], l)
      lines(ys, d(ys), type = "l")
      points(ys, d(ys), pch = pchs[i])
    }
    legend("topright", legend = legends, pch = pchs)
  }
}
