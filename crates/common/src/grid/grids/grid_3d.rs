use crate::prelude::*;

pub struct Grid3d<T: GridParam, const LAYER_COUNT: usize> {
    size: UVec2,
    layers: [Grid<T>; LAYER_COUNT],
}

impl<T: GridParam, const LAYER_COUNT: usize> Grid3d<T, LAYER_COUNT> {
    pub fn new_clone(size: impl Size2d, value: T) -> Self
    where T: Clone {
        let mut layers = Vec::new();
        for _ in 0..LAYER_COUNT {
            let layer = Grid::new_clone(size, value.clone());
            layers.push(layer);
        }

        Self {
            size: size.as_uvec2(),
            layers: layers.try_into().unwrap_or_else(|_v| panic!("Something went wrong!")),
        }
    }

    pub fn new_copy(size: impl Size2d, value: T) -> Self
    where T: Copy {
        let mut layers = Vec::new();
        for _ in 0..LAYER_COUNT {
            let layer = Grid::new_copy(size, value);
            layers.push(layer);
        }

        Self {
            size: size.as_uvec2(),
            layers: layers.try_into().unwrap_or_else(|_v| panic!("Something went wrong!")),
        }
    }

    pub fn new_default(size: impl Size2d) -> Self
    where T: Default {
        let mut layers = Vec::new();
        for _ in 0..LAYER_COUNT {
            let layer = Grid::new_default(size);
            layers.push(layer);
        }

        Self {
            size: size.as_uvec2(),
            layers: layers.try_into().unwrap_or_else(|_v| panic!("Something went wrong!")),
        }
    }

    pub fn new_fn(size: impl Size2d, f: impl Fn((usize, IVec2)) -> T) -> Self {
        let count = size.count();
        let mut layers = Vec::new();
        for index in 0..LAYER_COUNT {
            let mut cells = Vec::with_capacity(count);
            for coord in size.iter() {
                cells.push(f((index, coord)));
            }
            layers.push(Grid::<T> {
                size: size.as_uvec2(),
                cells,
            });
        }

        Self {
            size: size.as_uvec2(),
            layers: layers.try_into().unwrap_or_else(|_v| panic!("Something went wrong!")),
        }
    }

    pub fn blit_clone<ToLayerId: Into<usize>, FromLayerId: Into<usize>>(
        &mut self,
        to_layer_id: ToLayerId,
        to: impl Point2d,
        source: &Self,
        from_layer_id: FromLayerId,
        from: impl Point2d,
        size: impl Size2d,
    ) where
        T: Clone,
    {
        if let Some(to_layer) = self.get_grid_by_layer_mut(to_layer_id) {
            if let Some(from_layer) = source.get_grid_by_layer(from_layer_id) {
                to_layer.blit_clone(to, from_layer, from, size);
            }
        }
    }

    pub fn blit_copy<ToLayerId: Into<usize>, FromLayerId: Into<usize>>(
        &mut self,
        to_layer_id: ToLayerId,
        to: impl Point2d,
        source: &Self,
        from_layer_id: FromLayerId,
        from: impl Point2d,
        size: impl Size2d,
    ) where
        T: Copy,
    {
        if let Some(to_layer) = self.get_grid_by_layer_mut(to_layer_id) {
            if let Some(from_layer) = source.get_grid_by_layer(from_layer_id) {
                to_layer.blit_copy(to, from_layer, from, size);
            }
        }
    }

    pub fn blit_clone_from_2d<ToLayerId: Into<usize>>(
        &mut self,
        to_layer_id: ToLayerId,
        to: impl Point2d,
        source: &Grid<T>,
        from: impl Point2d,
        size: impl Size2d,
    ) where
        T: Clone,
    {
        if let Some(to_layer) = self.get_grid_by_layer_mut(to_layer_id) {
            to_layer.blit_clone(to, source, from, size);
        }
    }

    pub fn blit_copy_from_2d<ToLayerId: Into<usize>>(
        &mut self,
        to_layer_id: ToLayerId,
        to: impl Point2d,
        source: &Grid<T>,
        from: impl Point2d,
        size: impl Size2d,
    ) where
        T: Copy,
    {
        if let Some(to_layer) = self.get_grid_by_layer_mut(to_layer_id) {
            to_layer.blit_copy(to, source, from, size);
        }
    }

    pub fn blit_clone_to_2d<FromLayerId: Into<usize>>(
        &self,
        from_layer_id: FromLayerId,
        from: impl Point2d,
        dest: &mut Grid<T>,
        to: impl Point2d,
        size: impl Size2d,
    ) where
        T: Clone,
    {
        if let Some(from_layer) = self.get_grid_by_layer(from_layer_id) {
            dest.blit_clone(to, from_layer, from, size);
        }
    }

    pub fn blit_copy_to_2d<FromLayerId: Into<usize>>(
        &self,
        from_layer_id: FromLayerId,
        from: impl Point2d,
        dest: &mut Grid<T>,
        to: impl Point2d,
        size: impl Size2d,
    ) where
        T: Copy,
    {
        if let Some(from_layer) = self.get_grid_by_layer(from_layer_id) {
            dest.blit_copy(to, from_layer, from, size);
        }
    }

    #[inline]
    pub fn width(&self) -> u32 { self.size.width() }

    #[inline]
    pub fn height(&self) -> u32 { self.size.height() }

    #[inline]
    pub fn size(&self) -> UVec2 { self.size }

    #[inline]
    pub fn in_bounds(&self, pos: impl Point2d) -> bool { pos.is_valid(self.size()) }

    #[inline]
    pub fn is_layer<LayerId: Into<usize>>(&self, layer_id: LayerId) -> bool {
        layer_id.into() < self.layers.len()
    }

    #[inline]
    pub fn get_idx(&self, pos: impl Point2d) -> Option<usize> {
        if pos.is_valid(self.size()) {
            Some(self.get_idx_unchecked(pos))
        } else {
            None
        }
    }

    #[inline]
    pub fn get_idx_unchecked(&self, point: impl Point2d) -> usize { point.as_index(self.width() as usize) }

    #[inline]
    pub fn index_to_pt(&self, idx: usize) -> Option<UVec2> {
        let pt = self.index_to_pt_unchecked(idx);
        if pt.is_valid(self.size()) {
            Some(pt)
        } else {
            None
        }
    }

    #[inline]
    pub fn index_to_pt_unchecked(&self, idx: usize) -> UVec2 {
        let x = idx % self.width() as usize;
        let y = idx / self.width() as usize;
        UVec2::new(x as u32, y as u32)
    }

    #[inline]
    pub fn get<LayerId: Into<usize>>(&self, layer_id: LayerId, index: impl Point2d) -> Option<&T> {
        if let Some(layer) = self.get_grid_by_layer(layer_id) {
            layer.get(index)
        } else {
            None
        }
    }

    #[inline]
    pub fn get_mut<LayerId: Into<usize>>(
        &mut self,
        layer_id: LayerId,
        index: impl Point2d,
    ) -> Option<&mut T> {
        if let Some(layer) = self.get_grid_by_layer_mut(layer_id) {
            layer.get_mut(index)
        } else {
            None
        }
    }

    #[inline]
    pub fn get_unchecked<LayerId: Into<usize>>(&self, layer_id: LayerId, index: impl Point2d) -> &T {
        self.layers[layer_id.into()].get_unchecked(index)
    }

    #[inline]
    pub fn get_mut_unchecked<LayerId: Into<usize>>(
        &mut self,
        layer_id: LayerId,
        index: impl Point2d,
    ) -> &mut T {
        self.layers[layer_id.into()].get_mut_unchecked(index)
    }

    #[inline]
    pub fn set<LayerId: Into<usize>>(
        &mut self,
        layer_id: LayerId,
        index: impl Point2d,
        value: T,
    ) -> Option<T> {
        if let Some(layer) = self.get_grid_by_layer_mut(layer_id) {
            layer.set(index, value)
        } else {
            None
        }
    }

    #[inline]
    pub fn set_unchecked<LayerId: Into<usize>>(
        &mut self,
        layer_id: LayerId,
        index: impl Point2d,
        value: T,
    ) -> T {
        self.layers[layer_id.into()].set_unchecked(index, value)
    }

    pub fn get_grid_by_layer<LayerId: Into<usize>>(&self, layer_id: LayerId) -> Option<&Grid<T>> {
        let layer_id = layer_id.into();
        if self.is_layer(layer_id) {
            Some(&self.layers[layer_id])
        } else {
            None
        }
    }

    pub fn get_grid_by_layer_mut<LayerId: Into<usize>>(&mut self, layer_id: LayerId) -> Option<&mut Grid<T>> {
        let layer_id = layer_id.into();
        if self.is_layer(layer_id) {
            Some(&mut self.layers[layer_id])
        } else {
            None
        }
    }
}
