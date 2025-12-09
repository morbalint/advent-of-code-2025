#include <iostream>
#include <fstream>
#include <vector>
#include <sstream>
#include <cstdint>
#include <algorithm>

struct Point3D {
    int64_t x;
    int64_t y;
    int64_t z;

    Point3D(int64_t x, int64_t y, int64_t z) : x(x), y(y), z(z) {}
};

struct Edge {
    size_t point1_idx;
    size_t point2_idx;
    int64_t distanceSquared;

    Edge(size_t p1, size_t p2, int64_t distSq)
        : point1_idx(p1), point2_idx(p2), distanceSquared(distSq) {}

    bool operator<(const Edge& other) const {
        return distanceSquared < other.distanceSquared;
    }
};

std::vector<Point3D> readCoordinates(const std::string& filename) {
    std::vector<Point3D> coordinates;
    std::ifstream file(filename);
    std::string line;

    while (std::getline(file, line)) {
        std::stringstream ss(line);
        int64_t x, y, z;
        char comma;

        ss >> x >> comma >> y >> comma >> z;
        coordinates.emplace_back(x, y, z);
    }

    return coordinates;
}

int64_t calculateDistanceSquared(const Point3D& p1, const Point3D& p2) {
    int64_t dx = p2.x - p1.x;
    int64_t dy = p2.y - p1.y;
    int64_t dz = p2.z - p1.z;

    return dx * dx + dy * dy + dz * dz;
}

std::vector<Edge> findShortestEdges(const std::vector<Point3D>& points, size_t k = 10) {
    std::vector<Edge> edges;

    // Generate all edges
    for (size_t i = 0; i < points.size(); ++i) {
        for (size_t j = i + 1; j < points.size(); ++j) {
            int64_t distSq = calculateDistanceSquared(points[i], points[j]);
            edges.emplace_back(i, j, distSq);
        }
    }

    // Partial sort to get the k shortest edges
    size_t count = std::min(k, edges.size());
    std::partial_sort(edges.begin(), edges.begin() + count, edges.end());

    edges.erase(edges.begin() + count, edges.end());
    return edges;
}

class UnionFind {
private:
    std::vector<size_t> parent;
    std::vector<size_t> rank;

public:
    UnionFind(size_t n) : parent(n), rank(n, 0) {
        for (size_t i = 0; i < n; ++i) {
            parent[i] = i;
        }
    }

    size_t find(size_t x) {
        if (parent[x] != x) {
            parent[x] = find(parent[x]); // Path compression
        }
        return parent[x];
    }

    void unite(size_t x, size_t y) {
        size_t rootX = find(x);
        size_t rootY = find(y);

        if (rootX != rootY) {
            // Union by rank
            if (rank[rootX] < rank[rootY]) {
                parent[rootX] = rootY;
            } else if (rank[rootX] > rank[rootY]) {
                parent[rootY] = rootX;
            } else {
                parent[rootY] = rootX;
                rank[rootX]++;
            }
        }
    }

    std::vector<size_t> getComponentSizes() {
        std::vector<size_t> componentSize(parent.size(), 0);

        for (size_t i = 0; i < parent.size(); ++i) {
            size_t root = find(i);
            componentSize[root]++;
        }

        std::vector<size_t> result;
        for (size_t i = 0; i < componentSize.size(); ++i) {
            if (componentSize[i] > 0) {
                result.push_back(componentSize[i]);
            }
        }

        return result;
    }
};

std::vector<size_t> countNodesInGraphs(const std::vector<Point3D>& points,
                                        const std::vector<Edge>& edges) {
    UnionFind uf(points.size());

    // Connect points along the edges
    for (const auto& edge : edges) {
        uf.unite(edge.point1_idx, edge.point2_idx);
    }

    return uf.getComponentSizes();
}

std::vector<Edge> findAllEdgesSorted(const std::vector<Point3D>& points) {
    std::vector<Edge> edges;

    // Generate all edges
    for (size_t i = 0; i < points.size(); ++i) {
        for (size_t j = i + 1; j < points.size(); ++j) {
            int64_t distSq = calculateDistanceSquared(points[i], points[j]);
            edges.emplace_back(i, j, distSq);
        }
    }

    // Sort all edges by distance
    std::sort(edges.begin(), edges.end());

    return edges;
}

Edge findLastEdgeToConnectAll(const std::vector<Point3D>& points) {
    auto allEdges = findAllEdgesSorted(points);
    UnionFind uf(points.size());

    size_t numComponents = points.size();
    Edge lastEdge(0, 0, 0);

    for (const auto& edge : allEdges) {
        size_t root1 = uf.find(edge.point1_idx);
        size_t root2 = uf.find(edge.point2_idx);

        if (root1 != root2) {
            uf.unite(edge.point1_idx, edge.point2_idx);
            lastEdge = edge;
            numComponents--;

            if (numComponents == 1) {
                break;
            }
        }
    }

    return lastEdge;
}

int main() {
    auto coordinates = readCoordinates("input.txt");

    std::cout << "Read " << coordinates.size() << " coordinates\n";

    auto lastEdge = findLastEdgeToConnectAll(coordinates);

    const auto& p1 = coordinates[lastEdge.point1_idx];
    const auto& p2 = coordinates[lastEdge.point2_idx];

    std::cout << "\nLast edge to connect all points:\n";
    std::cout << "Edge: (" << p1.x << "," << p1.y << "," << p1.z << ") <-> "
              << "(" << p2.x << "," << p2.y << "," << p2.z << ")\n";
    std::cout << "Distance²: " << lastEdge.distanceSquared << "\n";
    std::cout << "product of x coordinates: " << p1.x * p2.x << "\n";

    return 0;
}