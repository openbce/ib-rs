# MyHCA

MyHCA dependent on `libverbs` and `libudev` to provide tools &amp; lib for the information of HCAs:

* lshca: a command line to list HCA information
* libhca: a library to get HCA information


## Install

```
$ sudo apt install -y libclang-dev libibverbs-dev libudev-dev libpci-dev pkg-config
$ cargo install --git https://github.com/xflops/myhca
```


## Example

```
$ lshca
----------------------------------------------
ID             : 15B3:0001
Model          : MT27800 Family [ConnectX-5]
Vendor         : Mellanox Technologies
FW             : 16.35.3006
Board          : MT_0000000008

    Name           Slot           Node GUID                Port GUID                LID            LinkType       State          PhysState
    mlx5_2         0000:b1:00.0   1070:fd03:0017:660c      -                        0              Eth            Down           Disabled
    mlx5_3         0000:b1:00.1   1070:fd03:0017:660d      1070:fd03:0017:660d      3              IB             Active         LinkUp
    mlx5_4         0000:b1:00.4   0000:0000:0000:0000      0000:0000:0000:0000      65535          IB             Down           LinkUp
    mlx5_5         0000:b1:00.5   0000:0000:0000:0000      0000:0000:0000:0000      65535          IB             Down           LinkUp


----------------------------------------------
ID             : 15B3:0116
Model          : MT42822 BlueField-2 integrated ConnectX-6 Dx network controller
Vendor         : Mellanox Technologies
FW             : 24.36.0356
Board          : MT_0000000732

    Name           Slot           Node GUID                Port GUID                LID            LinkType       State          PhysState
    mlx5_0         0000:4b:00.0   b83f:d203:006a:e616      b83f:d203:006a:e616      65535          IB             Down           Polling
    mlx5_1         0000:4b:00.1   b83f:d203:006a:e617      b83f:d203:006a:e617      4              IB             Active         LinkUp


```
