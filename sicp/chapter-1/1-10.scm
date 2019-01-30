(define (A x y)
  (cond ((= y 0) 0)
        ((= x 0) (* 2 y))
        ((= y 1) 2)
        (else (A (- x 1)
                 (A x (- y 1))))))

(define (print-A x y)
  (format #t "x ~D y ~D\n" x y)
  (cond ((= y 0) 0)
        ((= x 0) (* 2 y))
        ((= y 1) 2)
        (else (print-A (- x 1)
                       (print-A x (- y 1))))))

(define (print-F f N)
  (if (< N 2)
      (display (f 1))
      (print-F-iter f 1 (- N 1))))

(define (print-F-iter f n N)
  (format #t "~D " (f n))
  (if (<= N n)
      (display (f (+ N 1)))
      (print-F-iter f (+ n 1) N)))

(define (f n) (A 0 n))

;; 2 4 6 8 10 12 14 16 18 20 ...
(define (print-f N)
  (print-F f N))

(define (g n) (A 1 n))

;; 2 4 8 16 32 64 128 256 512 1024 ...
(define (print-g N)
  (print-F g N))

(define (h n) (A 2 n))

;; 2 4 16 65536 ...
(define (print-h N)
  (print-F h N))

(define (k n) (* 5 n n))

(define (print-k N)
  (print-F k N))
