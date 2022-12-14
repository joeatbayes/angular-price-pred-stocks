Stock Price Prediction using angular velocity changes in rust
By Joseph Ellsworth 2022-08-22

In my work attempting to predict future stock price movements
I have observed that pivotal buying or selling opportunities occur after
two actions.  For example stock prices drop for 3 weeks then stabilize
for 1 week.  At these pivotal points prices it can be a good time to 
buy or sell.  We need more data to determine if the current movements 
indicate a higher probability of a upward or downward movement in the
future. 

The theory is that certain patterns of these paired movements will result 
in predictable future price movements.  For example if stock prices drop for
3 months and then stabilize or rise slightly for 1 month then prices in the
next 10 days will tend to move upward but if prices drop for 3 months and
stabilize for 1 week they are more likely to resume a downward movement.
If we can find similar paired movements from the past they may help 
us predict future prices with a precision higher than the statistical
base rate.  Ideally it will deliver a precision high enough to act 
as a buy or sell signal or to confirm  independently generated signals.   
This application seeks to validate the theory by finding a set
of optimal time frames using multiple linear regression passes. 
It then uses geometric similarity scoring to select similar movements.
It then looks at subsequent price movements from those historically
similar movements to predict future price movements.  

The ML techniques used are  linear regression, K Nearest Neighbor,
Bayesian math, random permutation optimization.  


------------------------
Key Repository Files:
------------------------    
•   Main Code entry point: 
        https://github.com/joeatbayes/angular-price-pred-stocks/blob/main/predictor/src/main.rs

•   Main regression application & similarity scoring: 
        https://github.com/joeatbayes/angular-price-pred-stocks/blob/main/predictor/src/reg_fit.rs

•   Bars Parser / CSV: 
        https://github.com/joeatbayes/angular-price-pred-stocks/blob/main/predictor/src/bar_parser.rs

•   K Nearest Neighbor Matching using binary search
        https://github.com/joeatbayes/angular-price-pred-stocks/blob/main/predictor/src/angle_matcher.rs

•   Back test logic for simple trading test against classifier
        https://github.com/joeatbayes/angular-price-pred-stocks/blob/main/predictor/src/back_test.rs 

•   Config file for default settings of magic constants like how many bars to harvest
    how many neighbors to consider in initial harvest.  How many neighbors to keep, etc.
         https://github.com/joeatbayes/angular-price-pred-stocks/blob/main/predictor/src/config.rs


•   Optimizer seeking to find best look forward and minimum short trend line
    lengths.

----------------------------------------------------------------
Sample from the Initial Linear Regression slope fitting for SPY
----------------------------------------------------------------

         long  long      long    long     short  short      short   short intersect   fp dif
        slope   len    offset     end     slope    len     offset     end     angle     Perc
        ------- ----- --------- ------- --------- ----- --------- ------- --------- --------
        0.00397    43    73.301    1097   0.00568    14    83.658    1111      9.18    1.72%
        0.00400    53   258.638    6893   0.00076    17   307.605    6910    -33.20    2.38%
        0.00403    55   256.283    6893   0.00089    18   307.302    6911    -31.16    1.01%
        0.00403    39   113.654    2102  -0.00044    12   127.161    2114    -27.93    0.35%
        0.00414    37    77.080    4097  -0.00207    12    92.019    4109    -28.26    3.94%
        0.00417    40    77.769    4102   0.00217    12    88.749    4114     -7.15   -0.23%
        0.00417    40    77.769    4102   0.00341    13    88.128    4115     -1.24   -0.73%
        0.00424    51    99.160    1485   0.00554    15   115.854    1500      9.76   -2.71%
        0.00430    72    71.581    4118  -0.00274    24    95.224    4142    -32.31    5.85%
        0.00437    75    72.418    4125   0.00081    25    90.197    4150    -13.68    2.45%
        0.00445    42    76.739    4102   0.00411    14    87.754    4116      1.33    1.06%
        0.00464    65    71.906    4115  -0.00245    20    95.487    4135    -32.01   -4.23%
        0.00464    65    71.906    4115  -0.00223    21    95.318    4136    -30.90   -4.69%
        0.00478    49    70.729    4095  -0.00111    16    91.508    4111    -25.39    5.78%
        0.00480    48   240.163    6876   0.00517    16   292.752    6892      6.92    2.49%
        0.00503    45    97.456    1478   0.00003    12   117.669    1490    -26.19    3.00%
        0.00505    57   240.876    6887  -0.00283    17   319.514    6904     87.38    1.76%
        0.00505    57   240.876    6887  -0.00250    18   318.736    6905    -89.11    1.82%
        0.00530    47   239.832    6877   0.00737    14   290.090    6891     13.48    3.68%
        0.00537    42    96.910    1475  -0.00017    14   117.488    1489    -29.09    1.85%
        0.00545    45   239.239    6875   0.00672    13   287.808    6888     10.44   -5.88%

-----------------------------------------------
Sample from basic Stats for underlying bar file
-----------------------------------------------
    base stats simple hold N days=TradeStats {
        win_tot: 803.7904,
        win_cnt: 202,
        win_rat: 0.55342466,
        win_avg: 3.9791603,
        loss_cnt: 163,
        loss_tot: -851.67004,
        loss_rat: 0.44657534,
        loss_avg: 5.2249694,
        net_tot: -47.87964,
        net_avg: -0.1311771,
        appt: -0.13117719,
    }
    number of test_bar=365  # train bar=4444

---------------------------------------------
-- Sample from KNN Matching win loss analysis
---------------------------------------------
  -- TODO Include basic stats here compared to 
  -- base rate differential analysis also Include
  -- average win, average loss and sum of loss 
  -- sum of win. We are seeking a way to reconcile total
  -- amount that would have been one if every match had been
  -- traded versus total amount lost if every match had been
  -- traded.   
  
        trade stats=TradeStats {
            win_tot: 623.3998,
            win_cnt: 12,
            win_rat: 0.5714286,
            win_avg: 51.94998,
            loss_cnt: 9,
            loss_tot: -430.50012,
            loss_rat: 0.42857143,
            loss_avg: 47.833347,
            net_tot: 192.89966,
            net_avg: 9.185698,
            appt: 9.185699,
        }


-----------------------------------------
Sample from K Nearest Neighbor matching 
-----------------------------------------
             long  long      long    long    short short     short    short  intersect  fp dif      match
            slope   len    offset     end    slope   len    offset      end      angle    Perc      score
    ------ -------- ----- --------- ------- -------- ------ -------- -------- ---------- -------  ----------
   master  -0.776%    40   338.134    6848  -0.006%    10   279.503    6858       68.16    0.98%    0.0000
    ..     -0.446%    38    89.338    4055   2.057%     9    65.648    4064       76.47    4.16%    4.7609
    ..     -0.472%    32    96.290    2441   1.691%     9    75.363    2450       77.58    0.99%    4.7592
    ..     -0.438%    44   112.067    2387   1.585%     9    78.531    2396       78.42   -0.23%    4.7588
    ..     -1.037%    30   129.306    3965   1.846%     9    84.209    3974      -69.30    1.95%    4.7570
    ..     -0.437%    30   124.372    2178   0.999%    10    95.966    2188       72.80   -0.85%    4.7562
    ..     -0.534%    33   138.715    4677   0.427%     9   116.221    4686       62.30    1.75%    4.7532
    ..     -0.450%    34   139.273    2054   0.302%    10   112.532    2064       50.85    4.22%    4.7530
    ..     -0.468%    27    95.473    3996   0.024%     9    89.275    4005       25.49   -0.46%    4.7517
    ..     -0.525%    77   130.323    3998   0.068%    22    88.448    4020       37.65    0.82%    4.7516
    ..     -0.466%    28    94.281    2532  -0.162%     9    85.396    2541       15.47   -0.76%    4.7508
    ..     -0.462%    27   123.896    4364  -0.522%     9   111.655    4373       -0.84    0.33%    4.7492
            #win= 7  #loss=  4 win=63.64% loss=36.36%
            tot_win=   14.22% avg_win=   2.031% net_P&L= 11.919%
            tot_loss=  -2.30% avg_loss -0.574%

            long  long      long    long    short short     short    short intersect   fp dif      match
            slope   len    offset     end    slope   len    offset      end     angle     Perc      score
    ------ -------- ----- --------- ------- -------- ------ -------- -------- ---------- -------  ----------
   master   0.436%    30   111.472    4728  -0.105%    10   126.423    4738      -33.73   -2.39%    0.0000
    ..      0.435%    31    76.510    4090   0.971%    10    83.738    4100       21.62   -1.97%    0.8538
    ..      0.530%    47   239.832    6877   0.737%    14   290.090    6891       13.48    4.11%    0.8534
    ..      0.558%    30    87.831    4168   0.695%     9    97.460    4177        7.70   -3.44%    0.8534
    ..      0.490%    30    80.192    2466   0.678%     9    87.041    2475        9.06    0.92%    0.8530
    ..      0.450%    33   110.706    2089   0.515%    11   123.508    2100        6.32   -3.34%    0.8522
    ..      0.428%    31   241.105    6553   0.373%    10   268.018    6563       -0.59    0.02%    0.8516
    ..      0.563%    27    73.511    1088   0.107%     9    84.159    1097      -17.28    2.22%    0.8514
    ..      0.468%    31    83.268    2417   0.221%     9    88.743    2426      -10.59   -5.77%    0.8513
    ..      0.569%    36    97.362    1471  -0.041%    10   118.203    1481      -32.12   -4.11%    0.8509
            #win= 4  #loss=  5 win=44.44% loss=55.56%
            tot_win=    7.27% avg_win=   1.817% net_P&L= -11.371%
            tot_loss= -18.64% avg_loss -3.728%


                long  long      long    long    short short     short    short intersect   fp dif      match
            slope   len    offset     end    slope   len    offset      end     angle     Perc      score
    ------ -------- ----- --------- ------- -------- ------ -------- -------- ---------- -------  ----------
   master   0.853%    27   231.654    6859  -0.133%     9   289.059    6868      -84.83   -3.71%    0.0000
    ..      0.757%    34    69.023    4085   0.656%    11    82.540    4096        0.56    2.66%    0.8523
    ..      0.665%    27    96.853    1463   0.762%     9   110.271    1472        7.27   -0.84%    0.8522
    ..      0.558%    30    87.831    4168   0.695%     9    97.460    4177        7.70   -3.44%    0.8514
    ..      0.490%    30    80.192    2466   0.678%     9    87.041    2475        9.06    0.92%    0.8510
    ..      0.563%    27    73.511    1088   0.107%     9    84.159    1097      -17.28    2.22%    0.8494
    ..      0.468%    31    83.268    2417   0.221%     9    88.743    2426      -10.59   -5.77%    0.8493
    ..      0.496%    27   110.727    4725   0.087%     9   124.327    4734      -22.99   -2.04%    0.8490
            #win= 3  #loss=  4 win=42.86% loss=57.14%
            tot_win=    5.80% avg_win=   1.933% net_P&L= -6.297%
            tot_loss= -12.10% avg_loss -3.024%



------------------------------------------
sample raw data from KNN matching 
------------------------------------------
    BNDMatch {
        master: BNDPair {
            long_line: BestNumDayFit {
                sloper: 0.00067896856,
                num_ele: 61,
                err: 0.0054995078,
                offset: 118.91983,
                end_ndx: 3161,
                slope: 0.08099416,
            },
            short_line: BestNumDayFit {
                sloper: -0.00033224997,
                num_ele: 13,
                err: 0.003722864,
                offset: 122.25484,
                end_ndx: 3174,
                slope: -0.040604267,
            },
            angle: -6.9562035,
            fp_dif_rat: 0.010059732,
            score: 0.0,
        },
        matches: [
            BNDPair {
                long_line: BestNumDayFit {
                    sloper: 0.0006738162,
                    num_ele: 59,
                    err: 0.01786686,
                    offset: 108.253456,
                    end_ndx: 4454,
                    slope: 0.073890686,
                },
                short_line: BestNumDayFit {
                    sloper: 0.0015563975,
                    num_ele: 13,
                    err: 0.0026583597,
                    offset: 115.65132,
                    end_ndx: 4467,
                    slope: 0.18060437,
                },
                angle: 6.012009,
                fp_dif_rat: 0.00025347513,
                score: 1267.2136,
            },
            BNDPair {
                long_line: BestNumDayFit {
                    sloper: 0.0006931363,
                    num_ele: 59,
                    err: 0.01128587,
                    offset: 202.05121,
                    end_ndx: 5582,
                    slope: 0.13981946,
                },
                short_line: BestNumDayFit {
                    sloper: 0.0015727861,
                    num_ele: 13,
                    err: 0.0030207955,
                    offset: 205.70494,
                    end_ndx: 5595,
                    slope: 0.32467023,
                },
                angle: 10.028344,
                fp_dif_rat: 0.0062425425,
                score: 1267.2136,
            }
        ]
    }



-----------------
Design NOTES
-----------------
This analyzer attempts to predict upward or downward movements by looking
at similar movements in the past.    We use an assertion that longer
trends are better than shorter once provided the deviation from the trend 
is smallest.  To support this assertion the system seeks the longest trend
line between a mix and max of days which yields the smallest error in fit.
It applies this by working backwards from the current data point to find the
length of the short fit line and then starts from the beginning of the
shorter line working backwards to find the longer trend line. The intersection
of those two represents a vertex representation of a velocity change which 
can be measured as a slope of line 2 - slope of line 2 (or we could use trig 
angle from the two slopes). This yields 4 data points:  
   long Line slope,
   long Line length, 
   short Line slope, 
   Short line Length,  
   Angle of velocity change between short line and long line.  

   SEE: predictor/src/reg_fit.rs  struct BestNumDayFit and BNDPair

NOTE:  We could potentially skip the linear regression and simply
  describe a line as the slope between two bars and then use the 
  same error function to measure how the other data points
  deviate from that line.  It may not be as accurate as the 
  regression approach but it would be hundreds of times faster.


NOTE:  We are using linear regression for by comparing the lines.
  In most uses Linear regression will yield a offset + slope where 
  slope is a absolute value to increase.  Since we want to compare 
  these lines across time when the difference in base price is so 
  large that the absolute change would cause the slopes to be non-comparable
  we convert the slope into a fraction of base price which will allow
  the slopes across different time frames to be compared even if the 
  base price has doubled or tripled.  

To compute the similarity of any single line we can use a formula such as:
    SlopeDelta   = (slope Line - slope Compare Line)
    LengthDelta = (line Length - compare Line Length)
    similarityScore = slopeDelta / LengthDelta

    SEE: predictor/src/reg_fit.rs method find_best_fit_in_range

    A a more recent short trend line is combined with a longer 
    trend line to describe the pair which I saw in the stock analysis
    as A long term trend + a shorter term different movement which I 
    think represent a indicator to predict future prices. 
    
    SEE: predictor/src/reg_fit.rs method best_fit_angle
         predictor/src/reg_fit.rs method best-build_fit_angles

This is intended to allow the slope to be the main driver of similarity but 
if the lines are radically different lengths reduce the effect similarity.    
When comparing two tuple scores we can use a formula such as
     Sim1 = similarity(mainTuple.line1, compareTuple.line1)
     Sim2 = similarity(maiintuple.line2, compareTuple.line2)
     velChgAngle1 = slopeMainTuple.line2.slope - mainTuple.line1..slope
     velChgAngle2 = compareTuple.line2.slope - compareTuple.line1.slope
     velDelt = 1 / (VelChgAngle1 - velChgAngle2)
     netSim = ( sim1 + sim2)  / velDelt

     SEE: predictor/src/reg_fit.sr methods sim_score

NOTE: A alternative approach for comparing similarity is to compare 
  the angles of intersect as the primary key and the total length
  of the two trend lines as the secondary input. This has fewer
  weighting values and may yield a superior scoring mechanism. 


Note: To allow for large price changes we must compute a price movement range from
  min,max prices and compute slope as a fraction of that rather than a pure multiplier. 
  Otherwise for large price movements we could get values for velDelta greater than 0.    
  We encode this data for every bar in the series in a way that allows locating the set 
  of bars with angle that are most similar.  Once we have those bars we can look forward
  a period of time such as 10 bars and look for direction and magnitude of price movement. 
  If we average those price movements together we theoretically can use that data to 
  project a probable price movement for the same time frame from the current bar.  


-------------------------
-- Similarity Clustering
-------------------------
We will use a KNN similarityScore cluster where we use the similarity in slope, line length
  and angles to find the most similar data points.  We can use the the look at their future
  look at price movements combined with Bayesian math to compute the probability of a current
  data point from rising.   The amount of time we look forward is something we should be 
  able to change with a ML optimizer to find the best projection distance.   The KNN 
  similarity rules are a)  when projecting similarity can only look at prior points.
  B) no more than 30% (configurable) of the data can overlap with similar points.

  The data is organized by long slope which is the single largest weighted item in similarity
  scoring although we may want to change this in a ML optimizer.   We use bisect (binary search)
  to find the points with the greatest similarity then select incrementally data points moving
  each way to analyze the closest N neighbors moving out until data requirements are satisfied
  or the end of the array is reached.   Any nodes that violate rules
  such as too many overlapping data points are rejected during this selection process.  Each node
  is scored based on similarity to current node.  This process continues until N2 neighbors such 
  as 100 neighbors are selected.  The selected set is sorted based on maximum similarity and the 
  best N3 neighbors such as 10 are selected or rejecting any that have similarity scores that are
  too much lower than the highest similarity.    The projected look forward change is combined with
  Bayesian inspired math to determine probability of a move upward.


---------------------------------
-- Combining multiple time frames 
----------------------------------
  Note:  To maximize potential precision while dropping recall we may use multiple time frames such as
  a short time from from 15 to 30 days and a second short time frame from 30 to 60 days.  If the 
  theory holds true then confirming the movement of a short term trend with the a long term trend 
  should yield better predictive accuracy. 

