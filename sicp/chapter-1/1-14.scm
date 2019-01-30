(define (count-change amount)
  (cc amount 5))

(define (cc amount kinds-of-coins)
  (cond ((= amount 0) 1)
        ((or (< amount 0) (= kinds-of-coins 0)) 0)
        (else (+ (cc amount
                     (- kinds-of-coins 1))
                 (cc (- amount
                        (first-denomination kinds-of-coins))
                     kinds-of-coins)))))

(define (first-denomination kinds-of-coins)
  (cond ((= kinds-of-coins 1) 1)
        ((= kinds-of-coins 2) 5)
        ((= kinds-of-coins 3) 10)
        ((= kinds-of-coins 4) 25)
        ((= kinds-of-coins 5) 50)))

(define (print-tree amount)
  (print-cc amount 5 0))

(define (print-cc amount kinds-of-coins depth)
  (display (create-indent depth))
  (display (print-call amount kinds-of-coins))
  (display "\n")
  (cond ((= amount 0) 1)
        ((or (< amount 0) (= kinds-of-coins 0)) 0)
        (else (+ (print-cc amount
                           (- kinds-of-coins 1)
                           (+ depth 1))
                 (print-cc (- amount
                           (first-denomination kinds-of-coins))
                           kinds-of-coins
                           (+ depth 1))))))

(define (print-call amount kinds-of-coins)
   (format #f "(cc ~d ~d)" amount kinds-of-coins))

(define (create-indent depth)
  (define (iter n result)
    (cond ((< n 1) result)
          ((< n 2) (string-append result "|--"))
          (else (iter (- n 1)
                      (string-append result "|  ")))))
  (iter depth ""))
