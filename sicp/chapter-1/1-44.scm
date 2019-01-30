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

(define dx 1)

(define (smoothing f)
  (lambda (x) (/ (+ (f (- x dx)) (f x) (f (+ x dx)))
                 3)))

(define (smoothing-n f n)
  ((repeated smoothing n) f))
