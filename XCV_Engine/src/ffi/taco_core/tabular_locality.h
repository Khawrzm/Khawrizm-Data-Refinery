#pragma once
#include <string>
#include <vector>
#include <unordered_map>
#include <iostream>

namespace Khawrizm {
namespace TACO {

// Spatial Coordinates for O(1) Grid Mapping
struct CellCoord {
    int col;
    int row;
    bool operator==(const CellCoord& other) const { return col == other.col && row == other.row; }
};

struct CellRange {
    CellCoord head;
    CellCoord tail;
};

// The Four Pillars of Tabular Locality Compression
enum class CompressionPattern { RR, RF, FR, FF, SINGLE };

struct EdgeMetadata {
    CellCoord hRel;
    CellCoord hFix;
    CellCoord tRel;
    CellCoord tFix;
};

struct CompressedEdge {
    CellRange prec_range;
    CellRange dep_range;
    CompressionPattern pattern;
    EdgeMetadata meta;
};

class DirectedAcyclicGraph {
private:
    std::vector<CompressedEdge> compressed_edges;

public:
    void insert_dependency(const std::string& formula, const CellRange& prec, const CellRange& dep);
    void evaluate_graph();
    CompressionPattern detect_pattern(const CellRange& prec, const CellRange& dep);
};

} // namespace TACO
} // namespace Khawrizm
