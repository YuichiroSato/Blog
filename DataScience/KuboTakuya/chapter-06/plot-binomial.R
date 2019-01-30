qs <- seq(0.1, 0.9, by = 0.2)
xl <- "y"
yl <- "Probability"
legends <- paste0("q = ", qs)
for (N in 1:20) {
  title <- paste0("Binomial distribution, p(y|N = ", N, ", q)")
  plot(0, 0, type = "n", xlim = c(0, N), ylim = c(0.0, 1.0), main = title, xlab = xl, ylab = yl)
  for (i in 1:5) {
    lines(0:N, dbinom(0:N, N, qs[i]), type = "l")
    points(0:N, dbinom(0:N, N, qs[i]), pch = i)
  }
  legend("topright", legend = legends, pch = 1:5)
}
