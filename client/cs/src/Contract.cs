using generated = constellation.generated;
using RSharp;
using System.Collections.Generic;
using ObjectHandle = Teleportal.Client.Object.ObjectHandle;
using States = Teleportal.Client.Contract.Properties.States;

namespace Teleportal.Client.Contract
{
    public sealed class ContractDataHandle : OpaqueWrapper<ContractDataHandle>
    {
        public ContractDataHandle(Ptr<ContractDataHandle> inner) : base(inner, OwnershipSemantics.Owned) { }

        override protected void NativeDrop(Ptr<ContractDataHandle> inner)
        {
            generated.__Internal.ConstellationContractContractDataHandleDrop(inner.p);
        }
    }

    public sealed class ContractData : OpaqueWrapper<ContractData>
    {
        public ContractData(Ptr<ContractData> inner) : base(inner, OwnershipSemantics.SharedRef) { }

        override protected void NativeDrop(Ptr<ContractData> inner)
        {
            throw new System.InvalidOperationException("Unreachable code reached");
        }

        public ContractId Id
        {
            get
            {
                var cid = new Ptr<ContractId>(generated.__Internal.ConstellationContractContractDataId(this.Inner.Value.p));
                return new ContractId(cid);
            }
        }

        public IEnumerable<ObjectHandle> Objects
        {
            get => throw new System.Exception("todo");
        }
    }

    public sealed class ContractId : OpaqueWrapper<ContractId>
    {
        public ContractId(Ptr<ContractId> inner) : base(inner, OwnershipSemantics.SharedRef) { }

        override protected void NativeDrop(Ptr<ContractId> inner)
        {
            throw new System.InvalidOperationException("Unreachable code reached");
        }

        public System.ReadOnlySpan<byte> Name
        {
            get
            {
                var slice = generated.__Internal.ConstellationContractContractIdName(this.Inner.Value.p);
                unsafe
                {
                    return new System.ReadOnlySpan<byte>((byte*)slice.ptr, (int)slice.len);
                }
            }
        }

        public (ushort, ushort, ushort) Version
        {
            get
            {
                var version = generated.__Internal.constellation__contract__ContractId__version(this.Inner.Value.p);
                return (version.major, version.minor, version.patch);
            }
        }
    }
}
