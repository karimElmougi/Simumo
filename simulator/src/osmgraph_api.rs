use std::collections::HashMap;

use cpython::GILGuard;
use cpython::PyErr;
use cpython::PyObject;
use cpython::{PyDict, Python};

/// Osm Graph Api  Trait Interface
pub trait OsmGraphApi {
    /// Specifies the error type given by the Api implementation
    type ErrorType;

    /// Query that graph at a specified location
    ///
    /// # Arguments
    /// * `lon`  - longitude
    /// * `lat`  - latitude
    /// * `zoom` - size of the bonding
    ///
    fn query_graph(lon: f64, lat: f64, zoom: i64) -> Result<Box<Self>, Self::ErrorType>;

    /// Get the nodes of the queried graph
    fn get_nodes(&self) -> Result<HashMap<i64, (f64, f64)>, Self::ErrorType>;
    /// Get the adjacendies of the queried graph
    fn get_adjacencies(&self) -> Result<HashMap<i64, Vec<i64>>, Self::ErrorType>;
    /// Get the edges of the queried graph
    fn get_edges(&self) -> Result<HashMap<i64, i64>, Self::ErrorType>;
}

/// Python OsmGraphAPI
///
///  # Fields
///  * `gil`    - used to fetch the global python interpreter
///  * `locals` - collection of all global variable in a local scope
///               used to import the simumap lib
///
pub struct PythonOsmGraphApi {
    gil: GILGuard,
    locals: PyDict,
}

impl PythonOsmGraphApi {
    /// name of the python module
    const MODULE_NAME: &'static str = "simumap";

    /// calls a module that does a web query with overpy ( Overpass Python API)
    fn target_location(&self, lon: f64, lat: f64, zoom: i64) -> Result<(), PyErr> {
        let py = self.gil.python();
        py.eval(
            &format!(
                "{}.target_location({}, {}, {})",
                Self::MODULE_NAME,
                lon,
                lat,
                zoom
            ),
            None,
            Some(&self.locals),
        )?;
        Ok(())
    }
}

impl OsmGraphApi for PythonOsmGraphApi {
    type ErrorType = PyErr;

    fn query_graph(lon: f64, lat: f64, zoom: i64) -> Result<Box<Self>, PyErr> {
        let gil = Python::acquire_gil();

        let locals = {
            let py = gil.python();
            let simumap = py.import(Self::MODULE_NAME)?;
            let locals = PyDict::new(py);
            locals.set_item(py, Self::MODULE_NAME, simumap)?;
            locals
        };
        let result = Self { gil, locals };
        result.target_location(lon, lat, zoom)?;
        Ok(Box::new(result))
    }

    fn get_nodes(&self) -> Result<HashMap<i64, (f64, f64)>, PyErr> {
        let py = self.gil.python();
        let res: PyDict = py
            .eval(
                &format!("{}.get_nodes()", Self::MODULE_NAME),
                None,
                Some(&self.locals),
            )?
            .extract(py)?;

        res.items(py)
            .iter()
            .map(|x| {
                let id: i64 = x.0.extract(py)?;
                let val: (f64, f64) = x.1.extract(py)?;
                Ok((id, val))
            })
            .collect::<Result<HashMap<_, _>, _>>()
    }

    fn get_adjacencies(&self) -> Result<HashMap<i64, Vec<i64>>, Self::ErrorType> {
        let py = self.gil.python();
        let res: PyDict = py
            .eval(
                &format!("{}.get_adjacencies()", Self::MODULE_NAME),
                None,
                Some(&self.locals),
            )?
            .extract(py)?;

        res.items(py)
            .iter()
            .map(|x| {
                let beg: i64 = x.0.extract(py)?;
                let end: Vec<i64> = x.1.extract(py)?;
                Ok((beg, end))
            })
            .collect::<Result<HashMap<_, _>, _>>()
    }

    fn get_edges(&self) -> Result<HashMap<i64, i64>, PyErr> {
        let py = self.gil.python();
        let res: Vec<(PyObject, PyObject)> = py
            .eval(
                &format!("{}.get_edges()", Self::MODULE_NAME),
                None,
                Some(&self.locals),
            )?
            .extract(py)?;

        res.iter()
            .map(|x| {
                let beg: i64 = x.0.extract(py)?;
                let end: i64 = x.1.extract(py)?;
                Ok((beg, end))
            })
            .collect::<Result<HashMap<_, _>, _>>()
    }
}