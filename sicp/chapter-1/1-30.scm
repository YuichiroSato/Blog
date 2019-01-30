(define (sum term a0 next b)
  (define (iter a1 result)
    (if (> a1 b)
        result
        (iter (next a1) (+ (term a1) result))))
  (iter a0 0))
