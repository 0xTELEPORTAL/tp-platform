using ffi = constellation.generated.__Internal;
using States = Teleportal.Client.Contract.Properties.States;
using ObjectHandle = Teleportal.Client.Object.ObjectHandle;
using RSharp;


namespace Teleportal.Client
{
    partial class Baseline
    {

        // TODO(SER-406): This should all be autogenerated
        public States.State_U8 State(States.StateHandle_U8 state_handle)
        {

            var p = new Ptr<States.State_U8>(
                ffi.ConstellationBaselineBaselineStateU8(this.Inner.Value.p, state_handle.Inner.Value.p)
            );
            return new States.State_U8(p, OwnershipSemantics.SharedRef);
        }

        public States.State_U8 StateMut(States.StateHandle_U8 state_handle)
        {
            if (this.OwnershipSemantics == OwnershipSemantics.SharedRef)
            {
                throw new MutabilityException("`this` must be mutable!");
            }

            var p = new Ptr<States.State_U8>(
                ffi.ConstellationBaselineBaselineStateMutU8(this.Inner.Value.p, state_handle.Inner.Value.p)
            );
            return new States.State_U8(p, OwnershipSemantics.MutRef);
        }

        public States.State_I8 State(States.StateHandle_I8 state_handle)
        {
            var p = new Ptr<States.State_I8>(
                ffi.ConstellationBaselineBaselineStateI8(this.Inner.Value.p, state_handle.Inner.Value.p)
            );
            return new States.State_I8(p, OwnershipSemantics.SharedRef);
        }

        public States.State_I8 StateMut(States.StateHandle_I8 state_handle)
        {
            if (this.OwnershipSemantics == OwnershipSemantics.SharedRef)
            {
                throw new MutabilityException("`this` must be mutable!");
            }

            var p = new Ptr<States.State_I8>(
                 ffi.ConstellationBaselineBaselineStateMutI8(this.Inner.Value.p, state_handle.Inner.Value.p)
             );
            return new States.State_I8(p, OwnershipSemantics.MutRef);
        }

        public States.State_I16 State(States.StateHandle_I16 state_handle)
        {
            var p = new Ptr<States.State_I16>(
                ffi.ConstellationBaselineBaselineStateI16(this.Inner.Value.p, state_handle.Inner.Value.p)
            );
            return new States.State_I16(p, OwnershipSemantics.SharedRef);
        }

        public States.State_I16 StateMut(States.StateHandle_I16 state_handle)
        {
            if (this.OwnershipSemantics == OwnershipSemantics.SharedRef)
            {
                throw new MutabilityException("`this` must be mutable!");
            }

            var p = new Ptr<States.State_I16>(
                 ffi.ConstellationBaselineBaselineStateMutI16(this.Inner.Value.p, state_handle.Inner.Value.p)
             );
            return new States.State_I16(p, OwnershipSemantics.MutRef);
        }

        public States.State_F32 State(States.StateHandle_F32 state_handle)
        {
            var p = new Ptr<States.State_F32>(
                ffi.ConstellationBaselineBaselineStateF32(this.Inner.Value.p, state_handle.Inner.Value.p)
            );
            return new States.State_F32(p, OwnershipSemantics.SharedRef);
        }

        public States.State_F32 StateMut(States.StateHandle_F32 state_handle)
        {
            if (this.OwnershipSemantics == OwnershipSemantics.SharedRef)
            {
                throw new MutabilityException("`this` must be mutable!");
            }

            var p = new Ptr<States.State_F32>(
                  ffi.ConstellationBaselineBaselineStateMutF32(this.Inner.Value.p, state_handle.Inner.Value.p)
              );
            return new States.State_F32(p, OwnershipSemantics.MutRef);
        }

        public States.State_U64 State(States.StateHandle_U64 state_handle)
        {
            var p = new Ptr<States.State_U64>(
                ffi.ConstellationBaselineBaselineStateU64(this.Inner.Value.p, state_handle.Inner.Value.p)
            );
            return new States.State_U64(p, OwnershipSemantics.SharedRef);
        }

        public States.State_U64 StateMut(States.StateHandle_U64 state_handle)
        {
            if (this.OwnershipSemantics == OwnershipSemantics.SharedRef)
            {
                throw new MutabilityException("`this` must be mutable!");
            }

            var p = new Ptr<States.State_U64>(
                 ffi.ConstellationBaselineBaselineStateMutU64(this.Inner.Value.p, state_handle.Inner.Value.p)
             );
            return new States.State_U64(p, OwnershipSemantics.MutRef);
        }

        public States.State_String State(States.StateHandle_String state_handle)
        {
            var p = new Ptr<States.State_String>(
                ffi.ConstellationBaselineBaselineStateString(this.Inner.Value.p, state_handle.Inner.Value.p)
            );
            return new States.State_String(p, OwnershipSemantics.SharedRef);
        }

        public States.State_ObjectHandle State(States.StateHandle_ObjectHandle state_handle)
        {
            var p = new Ptr<States.State_ObjectHandle>(
                ffi.ConstellationBaselineBaselineStateObjectHandle(this.Inner.Value.p, state_handle.Inner.Value.p)
            );
            return new States.State_ObjectHandle(p, OwnershipSemantics.Owned);
        }

        public States.State_String StateMut(States.StateHandle_String state_handle)
        {
            if (this.OwnershipSemantics == OwnershipSemantics.SharedRef)
            {
                throw new MutabilityException("`this` must be mutable!");
            }

            var p = new Ptr<States.State_String>(
                ffi.ConstellationBaselineBaselineStateMutString(this.Inner.Value.p, state_handle.Inner.Value.p)
            );
            return new States.State_String(p, OwnershipSemantics.MutRef);
        }
    }
}
