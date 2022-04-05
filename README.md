1. Deposit
    - có collateral: lending - cho vay có thế chấp
    - ko collateral: farming - chỉ để pool ăn lãi suất (staking)

2. Borrow
    - cách tính total available được phép borrow tối đa là bao nhiêu ?


Ví Dụ:
1 NEAR = $5
1 ETH = $2
A: có 100 NEAR, $1000 USDT
=> deposit: 10 NEAR 
    -> ko collateral: 
        10 NEAR nó sẽ tạo trong pool, không được dùng làm tài sản thế chấp để đi vay
    -> có collateral: 
        10 NEAR nó sẽ tạo trong pool, quy đôi(time khi mà ông A đi borrow) NEAR => USDT để làm tài sản thế chấp.

=> borrow: vay 2 ETH => $4 , lãi suất theo APY (lãi suất gộp) = 10%/năm 
    -> nếu ông A borrow 2 ETH thì sẽ bị chịu lãi suất 10%/năm
    -> ông A sẽ toàn quyền sử dụng 2ETH làm gì thì làm :))

############################################################################################################################

Mô hình tính lãi suất:
    > Các khái niệm:
        - target_utilization: phần trăm lý tưởng cho việc sử dụng tài sản. Ví dụ: vay 80%(borrow 80%) so vs tổng cung(supplied)  
        - target_utilization_r: hằng số để sử dụng làm cơ sở cho việc tính toán APR kép ở mức sử dụng target_utilization.
        - max_utilization_r: hằng số để sử dụng làm cơ sở cho việc tính toán APR kép ở mức sử dụng 100% target_utilization.
        - reserve_ratio: phần trăm lãi thu được cho nearland(platform)

    > Định nghĩa 3 mức hay điểm sử dụng trên tổng cung (supplied):
        - utilization: 0% - vs rate  = 1
        - target utilization  - vs rate = target_utilization_r
        - 100% - vs rate = max_utilization_r

        1 + APR = r ** MS_PER_YEAR , vs MS_PER_YEAR số milisecond trong 1 năm 31536000000


    Based on the current supplied, reserved and borrowed balances, the current utilization is defined using the following formula:
    > Dựa trên số dư hiện tại được cung cấp, dự trữ và đi vay, việc sử dụng hiện tại được xác định theo công thức sau:
        utilization = (supplied + reserved) / borrowed

    > Tính toán current APR, chúng ta cần tìm ra hằng số r:
        if utilization <= target_utilization, 
            r = target_utilization_r * (utilization / target_utilization)
        if utilization > target_utilization, 
            r = target_utilization_r + (max_utilization_r - target_utilization_r) * (utilization - target_utilization) / (1 - target_utilization)

    > To calculate the amount of interest acquired for the duration of t milliseconds:
        interest(lãi suất) = (r ** t) * borrowed

    > Tính lãi suất phân phối dựa trên reserved(trích dự phòng), supplied(tổng cung), trên reserve_ratio(tỉ lệ cho việc trích dự phòng):
        - Lãi suất dự trữ:
            reserved_interest = interest * reserve_ratio
        - Dự trữ mới:
            new_reserved = reserved + reserved_interest
        - Tổng cung mới:
            new_supplied = supplied + (interest - reserved_interest)
        - Tổng borrowed mới:
            new_borrowed = borrowed + interest

=> Config sample:
    /// Represents an asset config.
    /// Example:
    /// 25% reserve, 80% target utilization, 12% target APR, 250% max APR, 60% vol
    /// no extra decimals, can be deposited, withdrawn, used as a collateral, borrowed
    /// JSON:
    /// ```json
    /// {
    ///   "reserve_ratio": 2500,
    ///   "target_utilization": 8000,
    ///   "target_utilization_rate": "1000000000003593629036885046",
    ///   "max_utilization_rate": "1000000000039724853136740579",
    ///   "volatility_ratio": 6000,
    ///   "extra_decimals": 0,
    ///   "can_deposit": true,
    ///   "can_withdraw": true,
    ///   "can_use_as_collateral": true,
    ///   "can_borrow": true
    /// }
    /// ```

    n = 31536000000 số milisecond trong 1 năm
    res = ((APR / 100) + 1) ** (1 / n)
    rate = res*10**27
    target_utilization_rate vs max_utilization_rate được tính bằng file apr_to_rate.py 



=> Mô hình lãi suất:
    1. Tính bước nhảy lãi suất (utilization)
        utilization = (supplied + reserved) / borrowed
    2. Dựa vào utilization, target_utilization, target_utilization_r, max_utilization_r để tinh ra r :
        if utilization <= target_utilization, 
            r = target_utilization_r * (utilization / target_utilization)
        if utilization > target_utilization, 
            r = target_utilization_r + (max_utilization_r - target_utilization_r) * (utilization - target_utilization) / (1 - target_utilization)

    3. Dựa vào r để tính ta lãi suất theo milliseconds: 
        interest(lãi suất) = (r ** t) * borrowed
    
    4. Dựa vào lãi suất thì tính được lãi suất phân phối dựa trên interest(lãi suất), reserved(trích dự phòng), supplied(tổng cung), reserve_ratio(tỉ lệ trích lập dự phòng):
        - Lãi suất dự trữ:
            reserved_interest = interest * reserve_ratio
        - Dự trữ mới:
            new_reserved = reserved + reserved_interest
        - Tổng cung mới:
            new_supplied = supplied + (interest - reserved_interest)
        - Tổng borrowed mới:
            new_borrowed = borrowed + interest

-

later right clip stable zero cupboard wrestle holiday between gold lava work