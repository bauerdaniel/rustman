//
// Copyright 2023 Daniel Bauer (bauerda@pm.me)
//

// Compile & Run
// g++ -o genpac genpac.cpp
// ./genpac

#include <cmath>
#include <iostream>
#include <fstream>
#include <sstream>
#include <string>

constexpr int DOTS_VERTICAL = 15;
constexpr int DOTS_HORIZONTAL = 56;

constexpr double GAP = 66.6666;

double round_to(double value, double precision = 1.0)
{
    return std::round(value / precision) * precision;
}

std::string double_to_str(double value)
{
    std::stringstream ss;
    ss.setf(std::ios::fixed, std::ios::floatfield);
    ss.precision(3);
    ss << value;
    return ss.str();
}

std::string circle_str(double x, double y)
{
    static int counter = 1;

    constexpr auto nl = "\n    ";
    constexpr auto style = R"(style="display:inline;fill:#ffaaa4;fill-opacity:1;stroke:none;stroke-width:1.11154;stroke-linecap:butt;stroke-linejoin:bevel;stroke-miterlimit:4;stroke-dasharray:none;stroke-opacity:1")";
    
    auto id = "id=\"pacmandot" + std::to_string(counter) + "\"";
    auto cx = "cx=\"" + double_to_str(x) + "\"";
    auto cy = "cy=\"" + double_to_str(y) + "\"";
    auto inkscape_label = "inkscape:label=\"Dot " + std::to_string(counter) + "\"";
    auto r = "r=\"10\"";

    std::stringstream ss;
    ss << "<circle"
        << nl << style
        << nl << id
        << nl << cx
        << nl << cy
        << nl << inkscape_label
        << nl << r
        << " />";

    ++counter;

    return ss.str();
}

int main(int argc, char* argv[])
{
    std::ofstream fs("dots.txt");

    for (size_t i = 0; i < DOTS_VERTICAL; i++) {
        for (size_t j = 0; j < DOTS_HORIZONTAL; j++) {

            auto x = 150. + j * GAP;
            auto y = 150. + i * GAP;

            fs << circle_str(x, y) << "\n";
        }
    }

    return 0;
}
