(define (deep-reverse ls)
  (if (pair? ls)
      (reverse (map deep-reverse ls))
      ls))
