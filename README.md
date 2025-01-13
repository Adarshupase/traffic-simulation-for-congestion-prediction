# Monitoring Traffic Lights and Predicting Future Congestion(This is simulation/real world map used by the predictor)

---

## **Template**
This document outlines the implementation of a traffic signal control system based on the concept of "view level" to predict future congestion.

---

## **Introduction**
 a traffic prediction system that leverages historical data and real-time traffic information at intersections. By predicting future congestion, traffic signals can be controlled more effectively.

The central idea is to use the concept of "view level," which focuses on the traffic relevant to a specific rider's journey, thereby simplifying the prediction process.

### **View Level**
A "view level" is a representation of traffic specific to a rider's concern. For example:
- If a rider travels from point A to point B, the system reduces the traffic network view to only the intersections relevant to that journey.

By limiting the view, we can predict traffic more efficiently, using:
- Real-time data.
- Historical data for intersections.

### **Key Approach**
1. Predict traffic at each node (intersection) based on historical and real-time data.
2. Use probabilities to estimate the flow of traffic in different directions.
3. Expand or limit the "view level" based on desired accuracy versus computational overhead.

---

## **Modeling Traffic Flow**

### **Equation for Traffic Prediction**
For a network of roads, the incoming traffic at an intersection (e.g., node B) can be calculated recursively:

**Example:**
- **Arriving Traffic at Node B (from up):**
    ```
    Arriving_traffic(b from up) =
        Leaving_traffic(c from down) +
        Arrival_probability(right to down at C) * Arrival_traffic(c from right)
    ```

- **Expanding Further:**
    ```
    Arrival_traffic(c from right) =
        Leaving_traffic(d from right) +
        Arrival_probability(up to right at D) * Arrival_traffic(d from up)
    ```

This recursive calculation continues until the desired node (e.g., A) is reached, where recursion stops.

### **Shortest Path Algorithm**
To optimize calculations, shortest path algorithms can be applied to limit computations to essential nodes.

---

## **Pseudo Code Overview**
```python
function predicted_arrival_traffic(node, direction, view_level):
    initialize traffic = 0

    match direction:
        case RIGHT:
            for each road connecting node from right:
                traffic += leaving_traffic(road, left) +
                          arrival_probability(left to right) * arrival_traffic(road, left, view_level) +
                          arrival_probability(down to right) * arrival_traffic(road, down, view_level) +
                          arrival_probability(up to right) * arrival_traffic(road, up, view_level)

        case LEFT:
            traffic += ... (similar logic for LEFT direction)
        case UP:
            traffic += ... (similar logic for UP direction)
        case DOWN:
            traffic += ... (similar logic for DOWN direction)

    return traffic
```

**Data Requirements:**
- Real-time data for `leaving_traffic`.
- Historical data to predict `arrival_probability` at intersections.

---

## **Current code state**

1. The values are hardcoded for both the directionProbability and directionDensity.
2. **directionDensity** is the real time traffic state(values are hardcoded for now).
3. **directionProbability** Provides an way to predict the future congestion levels (yet to implement).

This approach can enable riders to calculate the state of traffic signals independently, enhancing their journey planning without relying solely on broadcast signals.

