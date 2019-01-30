(define (double f)
  (lambda (x) (f (f x))))

(define (inc x) (+ x 1))

(define (main args)
  (display "(((double (double double)) inc) 5)\n")
  (format #t "~d\n" (((double (double double)) inc) 5)))
