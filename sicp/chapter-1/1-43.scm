(define (compose f g)
  (lambda (x) (f (g x))))

(define (repeated f n)
  (define (iter i result)
    (if (>= i n)
        result
        (iter (+ i 1) (compose f result))))
  (iter 1 f))

(define (square x)
  (* x x))

(define (main args)
  (display "((repeated square 2) 5)\n")
  (format #t "~d\n" ((repeated square 2) 5)))
