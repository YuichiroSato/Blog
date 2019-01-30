## �f��

### �����m���Ə����t���m���̃f��
[show-conditional-distribution.R](https://github.com/YuichiroSato/DataScience/blob/master/KuboTakuya/chapter-08/show-conditional-distribution.R)�ł͓����m���Ə����t���m���̊֌W�̃f����������悤�ɂ����B
`a` �� `b` �͋��ɓ񍀕��z���邪�A`b` �̔����m���� `a` �ɔ�Ⴗ��悤�ɂ����B
���s����ƈȉ��̏o�͂�������B
�����t���m���͓����m�������A�c��1��Ԃ񌩂��m�����z���Ƃ������Ƃ�������B

```
Joint distribution, p(a,b)
a\b | 0          1               2
0   | 0.10125    0.0225  0.00125
1   | 0.24       0.12    0.015
2   | 0.18375    0.1575  0.03375
3   | 0.045      0.06    0.02

Conditional distribution, p(b|a=i)
a\b | 0          1               2
0   | 0.81       0.18    0.01
1   | 0.64       0.32    0.04
2   | 0.49       0.42    0.09
3   | 0.36       0.48    0.16

Conditional distribution, p(a|b=i)
b\a | 0                  1                       2                       3
0   | 0.177631578947368  0.421052631578947       0.322368421052632       0.0789473684210526
1   | 0.0625     0.333333333333333       0.4375  0.166666666666667
2   | 0.0178571428571429         0.214285714285714       0.482142857142857       0.285714285714286
```

### �x�C�Y�̒藝�̃f��

[show-bayses-theorem.R](https://github.com/YuichiroSato/DataScience/blob/master/KuboTakuya/chapter-08/show-bayses-theorem.R)�ł̓x�C�Y�̒藝�̃T���v����������悤�ɂ����B
p(a,b) = p(a|b=i) * p(b=i) = p(b|a=i) * p(a=i)�����ĂƂ��B

```
Joint distribution, p(a,b)
a\b | 0          1               2
0   | 0.10125    0.0225  0.00125
1   | 0.24       0.12    0.015
2   | 0.18375    0.1575  0.03375
3   | 0.045      0.06    0.02

Joint distribution, p(a|b=i) * p(b=i)
a\b | 0          1               2
0   | 0.10125    0.0225  0.00125
1   | 0.24       0.12    0.015
2   | 0.18375    0.1575  0.03375
3   | 0.045      0.06    0.02

Joint distribution, p(b|a=i) * p(a=i)
a\b | 0          1               2
0   | 0.10125    0.0225  0.00125
1   | 0.24       0.12    0.015
2   | 0.18375    0.1575  0.03375
3   | 0.045      0.06    0.02
```

### ����m���̃f��

[plot-one-d-posterior.R](https://github.com/YuichiroSato/DataScience/blob/master/KuboTakuya/chapter-08/plot-one-d-posterior.R)�ł̓f�[�^����������鎞�̎���m�����v�Z����f����������悤�ɂ����B
�ޓx `p(y|q)` �͕��� `q` �̃|�A�\�����z������Ƃ��āA`q` �̎��O���z�͓񍀕��z�ɂ����B
����� `y` �� `q` �̓����m�����v�Z�ł��邩��A�x�C�Y�̒藝���玖��m�� `p(q|y)` ���v�Z�ł���B
�v�Z���Ă݂�ƁA�T�� `y` ���傫�Ȓl�����Ƃ��� `q` ���傫�Ȓl�����m���������Ȃ�̂�������B
`y` ���傫�Ȓl�����Ȃ�A`y` �𐶐������|�A�\�����z�̕��ς��傫�Ȓl�ł���ꍇ���������낤����A�Ó��Ȍ��ʂł��邱�Ƃ�������B

### ����m���̃f���Q

[plot-multi-d-posterior.R](https://github.com/YuichiroSato/DataScience/blob/master/KuboTakuya/chapter-08/plot-multi-d-posterior.R)�ł̓f�[�^����������ꍇ�̎���m�����v�Z����f����������悤�ɂ����B
�������̓f�[�^�̐���1���������A�����3�œ������Ƃ����B
�ޓx��3�� `y` ������ `q` �̃|�A�\�����z���瓾����m���̐ςɂȂ�B
��͂�f�[�^�ɑ傫�� `y` ���܂܂�Ă���ꍇ�� `q` ���傫�Ȓl�����Ƃ��Ɋm���������Ȃ�B

### ���O���z�Ɉ�l���z���g�����ꍇ�̃��g���|���X�@�T���v�����O

[plot-metro-uniform-prior.R](https://github.com/YuichiroSato/DataScience/blob/master/KuboTakuya/chapter-08/plot-metro-uniform-prior.R)�ł̓��g���|���X�@�̎��O���z�Ɉ�l���z���g�����ꍇ�̃��g���|���X�@�̃f����������悤�ɂ����B
�f�[�^ `Y` �𕽋� `5` ���U `1` �̐��K���z���琶�����A�ޓx�𕽋� `q` ���U `1` �̐��K���z���g���Čv�Z�� `p(q|Y)` �����g���|���X�@�ɂ���ăT���v�����O�����B
���O���z�ɂ͈�l���z�����肵�A�ޓx�������g���ăT���v�����O�����ꍇ�Ǝ��O���z���������ꍇ���ׂĂ݂��B
���O���z����l���z�̏ꍇ�͖ޓx�Ɏ��㕪�z�������Ă������Ȃ��Ă��������z�Ɏ�������̂�������B

### ���O���z�Ɉ�l���z�łȂ����z���g�����ꍇ�̃��g���|���X�@�T���v�����O

[plot-metro-prior.R](https://github.com/YuichiroSato/DataScience/blob/master/KuboTakuya/chapter-08/plot-metro-prior.R)�ł͎��O���z����l���z�łȂ��ꍇ�̃��g���|���X�@�̃f����������悤�ɂ����B
�f�[�^�� `Y` �͕��� `q` ���U `1` �̐��K���z���瓾����B
�������A�f�[�^�𐶐����鐳�K���z�̕��� `q` �� `3` �܂��� `7` �𒆐S�ɂ������U `0.1` �̐��K���z���瓾����B
�Ȃ̂ŁA`q` �̎��O���z�͈�l���z�ł͂Ȃ��B
`p(q|Y)` ���T���v�����O����̂ɁA�ޓx�Ƃ��Đ��K���z�ւ̓��Ă͂܂�̗ǂ��������g���ƁA`q` �͕��ϖ� `5` �̐��K���z�Ɏ�������B
���O���z�Ƃ��Ăӂ����Ԃ̐��K���z���g���ƁA�������ӂ����Ԃ̐��K���z�Ɏ�������B
�������A���O���z�Ƃ��Ĉ�l���z���g���A�ޓx�̕��ɂӂ����Ԃ̐��K���z���g���Ă����܂������B
