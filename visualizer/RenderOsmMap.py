import jinja2
from distutils.dir_util import copy_tree
import os
import shutil


def render_ol_map(map_output_path, city):
    # copy directory containing the outputmap and its ressources
    geocoder_path = "./ol-geocoder/template"
    map_output_path = "./output"
    if os.path.isdir(map_output_path):
        shutil.rmtree(map_output_path)  # in case the command was run before
    copy_tree(geocoder_path, map_output_path)

    # generate javascript file (OlMapTemplate.j2) from template so that:
    # - the map generated is zoomed at the requested city
    # - the map contains heatmap point to represent the selected metric
    template_env = jinja2.Environment(loader=jinja2.FileSystemLoader(map_output_path))
    template = template_env.get_template("OlMapTemplate.j2")
    map_rendered = template.render(initialLocation="\"" + city + "\"")
    with open(map_output_path + "/OlMapTemplate.js", "w") as text_file:
        print(map_rendered, file=text_file)
    os.remove(map_output_path + "/OlMapTemplate.j2")


def render_layout(map_output_path, timeline, metrics, legend):
    # copy directory containing the outputmap and its ressources
    layout_path = "./layoutVisualization"

    copy_tree(layout_path, map_output_path)

    # generate html layout from the file layout.j2 so that
    # - the timeline has the good minimum, maximum and interval incrementation,
    # - the metrics selector contains all the requested metrics
    # - the legend of the metrics si the one requested (not done for now)
    template_env = jinja2.Environment(loader=jinja2.FileSystemLoader(map_output_path))
    template = template_env.get_template('layout.j2')
    map_rendered = template.render(timeline=timeline, metrics=metrics, legend=legend)
    with open(map_output_path + "/layout.html", "w") as text_file:
        print(map_rendered, file=text_file)
    os.remove(map_output_path + '/layout.j2')


def render_visualization(city, timeline, metrics, legend):
    map_output_path = "./output"
    render_ol_map(map_output_path, city)
    render_layout(map_output_path, timeline, metrics, legend)

