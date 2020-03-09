#![allow(non_upper_case_globals)]

use std::{marker::PhantomData, ptr::NonNull};

use spine_sys::{
    spAttachment, spAttachmentType_SP_ATTACHMENT_MESH, spAttachmentType_SP_ATTACHMENT_REGION,
    spMeshAttachment, spRegionAttachment,
};

use super::{error::Error, result::Result};

pub enum Attachment<'a> {
    Region(RegionAttachment<'a>),
    Mesh(MeshAttachment<'a>),
    Other,
}

#[repr(transparent)]
pub struct RegionAttachment<'a>(
    pub(crate) NonNull<spRegionAttachment>,
    pub(crate) PhantomData<&'a ()>,
);

#[repr(transparent)]
pub struct MeshAttachment<'a>(
    pub(crate) NonNull<spMeshAttachment>,
    pub(crate) PhantomData<&'a ()>,
);

impl<'a> Attachment<'a> {
    pub(crate) fn from_pointer(pointer: *mut spAttachment) -> Result<Self> {
        let pointer = NonNull::new(pointer).ok_or(Error::NullPointer)?;

        Ok(match unsafe { pointer.as_ref().type_ } {
            spAttachmentType_SP_ATTACHMENT_REGION => {
                Attachment::Region(RegionAttachment(pointer.cast(), PhantomData))
            }

            spAttachmentType_SP_ATTACHMENT_MESH => {
                Attachment::Mesh(MeshAttachment(pointer.cast(), PhantomData))
            }

            _ => Attachment::Other,
        })
    }
}