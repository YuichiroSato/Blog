(define (expt b n)
  (define (expt-iter i acc1 acc2)
    (cond ((<= i 1) (* acc1 acc2))
          ((even? i) (expt-iter (/ i 2) (square acc1) acc2))
          (else (expt-iter (- i 1) acc1 (* acc1 acc2)))))
  (if (<= n 1)
      1
      (expt-iter (- n 1) b b)))
