Stock Price Prediction using angular velocity changes in rust
By Joseph Ellsworth 2022-08-22

In my work attempting to predict future stock price movements
I have observed that pivotal buying or selling opportunities occur after
two actions.  For example stock prices drop for 3 weeks then stabilize
for 1 week.  At these pivotal points prices it can be a good time to 
buy but we need more data to determine if the current movements 
indicate a higher probability of a upward or downward movement in the
future. 

The theory is that certain patterns of these paired movements will result 
in predictable future price movements.  For example if stock prices drop for
3 months and then stabilize or rise slightly for 1 month then prices in the
next 10 days will tend to move upward but if prices drop for 3 months and
stabilize for 1 week they are more likely to resume a downward movement.
If we can find similar paired movements from the past they may help 
us predict future prices with a precision higher than the statistical
base rate and ideally with a precision high enough to act as or confirm
a buy or sell signal.   

This application seeks to validate this theory by finding a set
of optimal time frames using geometric similarity to predict future
price movements.  It uses linear regression, K Nearest Neighbor and
Bayesian math to determine similarity and project the future price
movement data based on the past movement of prices with similar 
geometric properties.  If the theory proves valid then we should be
able to predict directional movement with greater accuracy than the 
underlying statistical base rate.


------------------------
Key Repository Files:
------------------------    
•	Main Code entry point: 
        https://github.com/joeatbayes/angular-price-pred-stocks/blob/main/predictor/src/main.rs

•	Main regression application & similarity scoring: 
        https://github.com/joeatbayes/angular-price-pred-stocks/blob/main/predictor/src/reg_fit.rs

•	Bars Parser / CSV: 
        https://github.com/joeatbayes/angular-price-pred-stocks/blob/main/predictor/src/bar_parser.rs

•   K Nearest Neighbor Matching using binary search
        

•   Optimizer seeking to find best look forward and minimum short trend line
    lengths.


•  Sample from the Initial Linear Regression slope fitting for SPY

         long  long      long    long     short  short    short   short  interset   fp dif
        slope   len    offset     end     slope    len   offset     end     angle     Perc
        ------- ----- --------- ------- --------- ------- ------- ------- --------- --------
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
    
    SEE: predictor/src/reg_fit.rs methodbest_fit_angle
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

