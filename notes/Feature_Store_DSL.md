# Feature Store DSL Design

## 1. Feature

A feature represents an individual measurable/computed value.

## 2. Feature View

A feature view groups related features for an entity.

Example:

feature_view location_features {
    entity: land_parcel_id
    source: land_parcels

    ...
}

## 3. Entity

An entity uniquely identifies the real-world object
for which features are stored.

Example:
land_parcel_id

## 4. Raw Features

Features directly obtained from a source.

Example:

feature latitude {
    type: float64
    source: land_parcels.latitude
}

## 5. Computed Features

Features derived from other features.

Example:

feature connectivity_score {
    type: float64
    compute: weighted_sum(
        highway_distance,
        railway_distance,
        airport_distance
    )
}

## 6. Dependency Inference

Dependencies are inferred automatically from feature
references inside compute expressions.

No explicit depends_on field is required.

## 7. Feature DAG

Each feature becomes a node.

A dependency creates a directed edge.

A → B means A depends on B.

## 8. Cycle Detection

Cyclic dependencies are invalid because the computation
order cannot be determined.

Example:

A → B
B → A

The compiler must reject this.

## 9. Execution Order

A topological ordering of the feature DAG determines
a valid feature computation order.